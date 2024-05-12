// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use stable_test::{batch_generate_account,read_account_info,read_keypair};

use fastcrypto::encoding::Encoding;
use fastcrypto::hash::HashFunction;
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    encoding::Base64,
    secp256k1::Secp256k1KeyPair,
    secp256r1::Secp256r1KeyPair,
    traits::{EncodeDecodeBase64, KeyPair},
};
// use rand::{rngs::StdRng, SeedableRng};
use shared_crypto::intent::{Intent, IntentMessage};
use futures::{future, stream::StreamExt};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, Command, Transaction, TransactionData},
        digests::TransactionDigest,
    },
    SuiClientBuilder,
};
use sui_types::crypto::Signer;
use sui_types::crypto::SuiSignature;
use sui_types::crypto::ToFromBytes;
use sui_types::signature::GenericSignature;
use sui_types::{
    base_types::SuiAddress,
    crypto::{get_key_pair_from_rng, SuiKeyPair},
};



#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //batch_generate_account(100, "./address1.json");
    let sui_client = SuiClientBuilder::default().build_testnet().await?;

    let coin_type = "0x2::sui::SUI".to_string();
    let data=read_account_info("./address1.json")?;
    let client = SuiClientBuilder::default().build_testnet().await?;
    let key: SuiKeyPair=read_keypair(&data[0].keypair).unwrap();
    let sender: SuiAddress=(&key.public()).into();
    let second=read_keypair(&data[0].keypair).unwrap();
    let receiver: SuiAddress=(&second.public()).into();
    let coins_stream=client.coin_read_api().get_coins_stream(sender,Some(coin_type));
    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_0_000_000))
        .boxed();

    let coin = coins.next().await.unwrap();

    let mut ptb = ProgrammableTransactionBuilder::new();

    let split_coint_amount = ptb.pure(1000u64)?; // note that we need to specify the u64 type

    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));

    let argument_address = ptb.pure(receiver)?;
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
    let gas_price = client.read_api().get_reference_gas_price().await?;

    let builder = ptb.finish();


    let tx_data=TransactionData::new_programmable(sender, vec![coin.object_ref()], builder, 300000, gas_price);

    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data);
    // let dat=tx_data.into();
    //let raw_tx = bcs::to_bytes(&intent_msg).expect("bcs should not fail");
    //let mut hasher=sui_types::crypto::DefaultHash::default();
    
    //hasher.update(raw_tx.clone());
    // let digest = hasher.finalize().digest;


    // let sui_sig=key.sign(&digest);

    // let res = sui_sig.verify_secure(
    //     &intent_msg,
    //     sender,
    //     sui_types::crypto::SignatureScheme::ED25519,
    // );
    // assert!(res.is_ok());


    //  // execute the transaction.
    //  let transaction_response = client
    //  .quorum_driver_api()
    //  .execute_transaction_block(
    //      sui_types::transaction::Transaction::from_generic_sig_data(
    //          intent_msg.value,
    //          vec![GenericSignature::Signature(sui_sig)],
    //      ),
    //      SuiTransactionBlockResponseOptions::default(),
    //      None,
    //  )
    //  .await?;

    // let tx_data: TransactionData=TransactionData::new_programmable(kind, sender, gas_payment, gas_budget, gas_price)

    // let x=Keystore::from(key);
    
    // FileBasedKeystore::new(path)
    
    
    //key.borrow()
    // Keystore::from(value)
    
    // println!("{:?}");

    // let mut file = File::create("account.json").unwrap();
    // //let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());


    // let (address,key)=get_account_key_pair();

    // file.write_all(buf)

    //println!("{:?}",json_str);

    // // 1) get the Sui client, the sender and recipient that we will use
    // // for the transaction, and find the coin we use as gas
    // let (sui, sender, recipient) = setup_for_write().await?;

    // // we need to find the coin we will use as gas
    // let coins = sui
    //     .coin_read_api()
    //     .get_coins(sender, None, None, None)
    //     .await?;
    // let coin = coins.data.into_iter().next().unwrap();

    // // programmable transactions allows the user to bundle a number of actions into one transaction
    // let mut ptb = ProgrammableTransactionBuilder::new();

    // // 2) split coin
    // // the amount we want in the new coin, 1000 MIST
    // let split_coint_amount = ptb.pure(1000u64)?; // note that we need to specify the u64 type
    // ptb.command(Command::SplitCoins(
    //     Argument::GasCoin,
    //     vec![split_coint_amount],
    // ));

    // // 3) transfer the new coin to a different address
    // let argument_address = ptb.pure(recipient)?;
    // ptb.command(Command::TransferObjects(
    //     vec![Argument::Result(0)],
    //     argument_address,
    // ));

    // // finish building the transaction block by calling finish on the ptb
    // let builder = ptb.finish();

    // let gas_budget = 5_000_000;
    // let gas_price = sui.read_api().get_reference_gas_price().await?;
    // // create the transaction data that will be sent to the network
    // let tx_data = TransactionData::new_programmable(
    //     sender,
    //     vec![coin.object_ref()],
    //     builder,
    //     gas_budget,
    //     gas_price,
    // );

    // // 4) sign transaction
    // let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    // let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    // // 5) execute the transaction
    // print!("Executing the transaction...");
    // let transaction_response = sui
    //     .quorum_driver_api()
    //     .execute_transaction_block(
    //         Transaction::from_data(tx_data, vec![signature]),
    //         SuiTransactionBlockResponseOptions::full_content(),
    //         Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    //     )
    //     .await?;
    // print!("done\n Transaction information: ");
    // println!("{:?}", transaction_response);

    // let coins = sui
    //     .coin_read_api()
    //     .get_coins(recipient, None, None, None)
    //     .await?;

    // println!(
    //     "After the transfer, the recipient address {recipient} has {} coins",
    //     coins.data.len()
    //);
    Ok(())
}