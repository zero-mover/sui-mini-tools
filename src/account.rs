

use std::fs::{self,File};
use std::io::Write;
use std::path::{Path,PathBuf};
use serde::{Serialize, Deserialize};
use sui_types::{ 
    base_types::SuiAddress, 
    crypto::{get_key_pair, EncodeDecodeBase64, SuiKeyPair},
};
use shared_crypto::intent::Intent;
use fastcrypto::hash::{HashFunction,HashFunctionWrapper};


use anyhow::anyhow;

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_keys::keypair_file::write_keypair_to_file;

use sui_sdk::{
    rpc_types::{SuiTransactionBlockResponseOptions,SuiTransactionBlockResponse},
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, Command, Transaction, TransactionData},
        quorum_driver_types::ExecuteTransactionRequestType,
    },
    SuiClientBuilder,
    SuiClient
};

use futures::{future, stream::StreamExt};

#[derive(Serialize, Deserialize,Debug)]
pub struct AddressKeyPair {
    pub account: String,
    pub keypair: String
}
#[tokio::main]
pub  async fn build_tranfer_tx(
    client: SuiClient,
    path: &PathBuf,
    recipient: SuiAddress,
    amount: u64,
    gas_buget: u64,
    coin_type: String
)-> Result<SuiTransactionBlockResponse, anyhow::Error> {
    //check path valid
    let keystore = FileBasedKeystore::new(path).unwrap();
    // default use first account 
    let sender=keystore.addresses()[0];
    
    //let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;
   // let sender_addr: SuiAddress=(&sender.public()).into();
    let coins_stream=client.coin_read_api().get_coins_stream(sender,Some(coin_type));
    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_0_000_000))
        .boxed();
    let coin =  coins.next().await.unwrap();
    let mut ptb = ProgrammableTransactionBuilder::new();

    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb.pure(amount).unwrap(); // note that we need to specify the u64 type
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));

    // 3) transfer the new coin to a different address
    let argument_address = ptb.pure(recipient).unwrap();
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
    let gas_price = client.read_api().get_reference_gas_price().await.unwrap();
    // finish building the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let tx_data: TransactionData=TransactionData::new_programmable(sender, vec![coin.object_ref()], builder, gas_buget, gas_price);
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    let transaction_response: sui_sdk::rpc_types::SuiTransactionBlockResponse = client
    .quorum_driver_api()
    .execute_transaction_block(
        Transaction::from_data(tx_data, vec![signature]),
        SuiTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    )
    .await?;
    Ok(transaction_response)

}

pub fn batch_generate_account(num_keypairs: usize,path: &str)-> Result<(), anyhow::Error>{
    let mut file= if !Path::new(path).exists(){
        File::create(path).unwrap()
    }else{
        File::open(path).unwrap()
    };
    let mut keypair: Vec<AddressKeyPair> = Vec::new();
    for _ in 0..num_keypairs {
        let kp_ed = SuiKeyPair::Ed25519(get_key_pair().1);
        let addr_ed: SuiAddress = (&kp_ed.public()).into();
        let address_keypair = AddressKeyPair {
            account: addr_ed.to_string(),
            keypair: kp_ed.encode_base64(),
        };
        keypair.push(address_keypair)
    };
    let json_str = serde_json::to_string_pretty(&keypair)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}

pub fn batch_generate_account_to_file(num_keypairs: usize,path: &str,key_store_path: &str)-> Result<(), anyhow::Error>{
    fs::create_dir_all(key_store_path).unwrap();
    let mut file=if !Path::new(path).exists(){
        File::create(path).unwrap()
    }else{
        File::open(path).unwrap()
    };

    let dir_path = Path::new(key_store_path);
    let mut keypair: Vec<AddressKeyPair> = Vec::new();


    for i in 0..num_keypairs {
        let kp_ed = SuiKeyPair::Ed25519(get_key_pair().1);
        let addr_ed: SuiAddress = (&kp_ed.public()).into();

        let key_path=dir_path.join(format!("{}.key", i));
        let address_keypair = AddressKeyPair {
            account: addr_ed.to_string(),
            keypair: key_path.clone().into_os_string().into_string().unwrap(),
        };
        write_keypair_to_file(&kp_ed,&key_path).unwrap();
        keypair.push(address_keypair)
    };


    let json_str = serde_json::to_string_pretty(&keypair)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}



pub fn read_account_info(path: &str)-> Result<Vec<AddressKeyPair>, anyhow::Error>{
    // let mut keypair: Vec<AddressKeyPair> = Vec::new();
    // if !Path::new(path).exists(){
    //     return Err("file no exist".into());
    // }else{
    //     File::open(path);
    // };

    let file=fs::read_to_string(path).unwrap();
    // let mut file: Result<File, std::io::Error> =File::open(path);
    let keypair: Vec<AddressKeyPair>=serde_json::from_str(&file).unwrap();
    Ok(keypair)
}


pub fn read_keypair(keypair: &str) -> anyhow::Result<SuiKeyPair> {
    // let contents = std::fs::read_to_string(path)?;
    SuiKeyPair::decode_base64(keypair.trim()).map_err(|e| anyhow!(e))
}