
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use tokio::try_join;
use stable_test::{fetch_coin, read_account_info, send_tranfer_tx, Config, TransferParams};
use sui_sdk::{
    SuiClientBuilder,
    types::base_types::SuiAddress,
};
use std::{thread, sync::Arc};
use futures::executor::block_on;

#[tokio::main]
 async fn main() ->Result<(),anyhow::Error>{
    let config=Config::default();
    let mut handles = vec![];
    let params=TransferParams::default();
    //let client=SuiClientBuilder::default().build(config.rpc).await?;
    let accounts=read_account_info(&config.json_address_file)?;
    let client=SuiClientBuilder::default().build(config.rpc.clone()).await?;
   
    // let key_store_file=Arc::new(config.key_store_file);
    // let x= client.as_ref();
    //let client_clone = client.clone();
    // let client_ref=Arc::clone(&client);

    for threadId in 0..config.thread {
        let sender: SuiAddress =accounts[threadId].account.parse()?;
        let recipient: SuiAddress =accounts[threadId+params.step].account.parse()?;
        let coin=fetch_coin(&client, &sender).await?.unwrap();
        let handle=send_tranfer_tx(
            &client,
            &config.key_store_file,
            sender,
            recipient,
            params.amount,
            coin,
            params.gas_budget,
        );
        // let handle = tokio::spawn(
            
        // );
        handles.push(handle);
        // let key_store_file=config.key_store_file.clone();
        // let handle=send_tranfer_tx(&client, &config.key_store_file, sender, recipient, params.amount, coin, params.gas_budget);
        // handles.push(handle);
    };
    
    // block_on(handles[0]);

    for handle in handles {
        //block_on(handle.await)
        let x=handle.await?;
    }
    Ok(())
}


