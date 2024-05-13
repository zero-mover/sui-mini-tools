use serde_json::from_str;
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use stable_test::{batch_generate_account, batch_generate_account_to_file, build_tranfer_tx, read_account_info, read_keypair,batch_generate_account_to_key_store_file};
use sui_sdk::{SuiClientBuilder,types::base_types::{ObjectID, SuiAddress}};
use std::{path::PathBuf, str::FromStr};
use futures::{future, stream::StreamExt};
use tokio::task;

use sui_keys::keystore::{self, AccountKeystore, FileBasedKeystore};


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //batch_generate_account_to_key_store_file(100,"./address_data.json","./key_store.keystore")?;
    let accounts=read_account_info("./address_data.json")?;


// //    let _ =batch_generate_account_to_file(100,"./address_info.json","./key_store/");
    let client = SuiClientBuilder::default().build_testnet().await?;

    let keystore = FileBasedKeystore::new(&PathBuf::from("./key_store.keystore"))?;
    // default use first account 
    let sender=keystore.addresses();
    let ss:SuiAddress=accounts[0].account.parse()?;
    let xd=keystore.get_key(&ss).unwrap().copy();
    let addr_ed:SuiAddress=(&xd.public()).into();

    // let addr_ed: SuiAddress =xd.into();

    println!("{:?} {:?}",addr_ed,ss);


   //;
    //println!("{:?}",keystore.get_key(&ss).unwrap());


//     // let sender_addr: SuiAddress =from_str(r#""0x7db19e51e43548723854bc1f0dca86f2138fda6ad9a4f404e48c86d4f68b5f29""#)?;

//     let arr=read_account_info("/Users/lizhengda/myspace/learning_rust/stable_test/address_info.json").unwrap();
//     let sender: SuiAddress=(&arr[0].account).parse()?;
//     let receiver: SuiAddress=(&arr[1].account).parse()?;
// //    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;
// //    let sender_addr: SuiAddress=(&sender.public()).into();
//     let coins_stream=client.coin_read_api().get_coins_stream(sender,Some("0x2::sui::SUI".to_string()));
//     let mut coins = coins_stream
//         .skip_while(|c| future::ready(c.balance < 5_0_000_000))
//         .boxed();
//     let coin =  coins.next().await.unwrap();

//     let resopnse=build_tranfer_tx(&client, "/Users/lizhengda/myspace/learning_rust/stable_test/key_store/0.keystore", receiver, 5_0_000_000, coin, 30000000).await.unwrap();
//     //std::thread::sleep(std::time::Duration::from_secs(1));
//     println!("{:?}",resopnse);

    
    Ok(())
}