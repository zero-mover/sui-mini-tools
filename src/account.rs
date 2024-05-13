

use std::fs::{self,File};
use std::io::Write;
use std::path::{Path,PathBuf};
use serde::{Serialize, Deserialize};
use sui_sdk::rpc_types::Coin;
use sui_types::{ 
    base_types::SuiAddress, 
    crypto::{get_key_pair, EncodeDecodeBase64, SuiKeyPair},
};
use shared_crypto::intent::Intent;
use anyhow::{anyhow, Ok};

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiTransactionBlockResponseOptions,SuiTransactionBlockResponse},
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, Command, Transaction, TransactionData},
        quorum_driver_types::ExecuteTransactionRequestType,
    },
    SuiClient
};

#[derive(Serialize, Deserialize,Debug)]
pub struct AddressKeyPair {
    pub account: String,
    pub keypair: String
}

// pub async fn send_move_call_tx(
//     client: &SuiClient,
//     path: &str,
//     sender: SuiAddress,
//     recipient: SuiAddress,
//     object_params:Vec<ObjectID>,
//     pure_params: Vec<>

// ){

// }


pub  async fn send_tranfer_tx(
    client: &SuiClient,
    path: &str,
    sender: SuiAddress,
    recipient: SuiAddress,
    amount: u64,
    coin: Coin, 
    gas_buget: u64,
)-> Result<SuiTransactionBlockResponse, anyhow::Error> {
    //check path valid
    if  !Path::new(path).exists() {
        return Err(anyhow!("key store file no exist"))
    }
    let key_store_file=PathBuf::from(path);

    let keystore = FileBasedKeystore::new(&key_store_file).expect("Failed to  key store file convert FileBasedKeystore struct");


    let mut ptb = ProgrammableTransactionBuilder::new();

    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb.pure(amount)?; // note that we need to specify the u64 type
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));

    // 3) transfer the new coin to a different address
    let argument_address = ptb.pure(recipient)?;
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
    let gas_price = client.read_api().get_reference_gas_price().await?;
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



pub fn batch_create_account(num_keypairs: usize,json_path: &str,key_store_file: &str)-> Result<Vec<AddressKeyPair>, anyhow::Error>{
    //
    let mut file= if !Path::new(json_path).exists(){
        File::create(json_path)?
    }else{
        File::open(json_path)?
    };
    let mut keystores=FileBasedKeystore::new(&PathBuf::from(key_store_file))?;
    let mut json_pair_arr: Vec<AddressKeyPair> = Vec::new();

    for _ in 0..num_keypairs {
        let kp_ed = SuiKeyPair::Ed25519(get_key_pair().1);
        let addr_ed: SuiAddress = (&kp_ed.public()).into();
        let address_keypair = AddressKeyPair {
            account: addr_ed.to_string(),
            keypair: kp_ed.encode_base64(),
        };
        json_pair_arr.push(address_keypair);
        keystores.add_key(None, kp_ed)?;
    };
    let json_str = serde_json::to_string_pretty(&json_pair_arr)?;
    file.write_all(json_str.as_bytes())?;
    Ok(json_pair_arr)
}




pub fn read_account_info(path: &str)-> Result<Vec<AddressKeyPair>, anyhow::Error>{
    let file=fs::read_to_string(path)?;
    let keypair: Vec<AddressKeyPair>=serde_json::from_str(&file)?;
    Ok(keypair)
}


pub fn read_keypair(keypair: &str) -> Result<SuiKeyPair,anyhow::Error> {
    // let contents = std::fs::read_to_string(path)?;
    SuiKeyPair::decode_base64(keypair.trim()).map_err(|e| anyhow!(e))
}