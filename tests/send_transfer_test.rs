use stable_test::{send_tranfer_tx,batch_create_account};
use sui_sdk::{
    SuiClientBuilder,
    SuiClient,
    rpc_types::{Coin, SuiObjectDataOptions},
    types::base_types::{SuiAddress,ObjectID}
};
use futures::{future, stream::StreamExt};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use tokio::test;
use serde_json::json;
use reqwest::Client;
use std::time::Duration;
use anyhow::bail;
use std::str::FromStr;
use std::fs;


pub const SUI_FAUCET: &str = "https://faucet.testnet.sui.io/v1/gas"; // testnet faucet

#[derive(serde::Deserialize)]
struct FaucetResponse {
    task: String,
    error: Option<String>,
}


#[allow(unused_assignments)]
pub async fn request_tokens_from_faucet(
    address: SuiAddress,
    sui_client: &SuiClient,
) -> Result<(), anyhow::Error> {
    let address_str = address.to_string();
    let json_body = json![{
        "FixedAmountRequest": {
            "recipient": &address_str
        }
    }];

    // make the request to the faucet JSON RPC API for coin
    let client = Client::new();
    let resp = client
        .post(SUI_FAUCET)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;
    println!(
        "Faucet request for address {address_str} has status: {}",
        resp.status()
    );
    println!("Waiting for the faucet to complete the gas request...");
    let faucet_resp: FaucetResponse = resp.json().await?;

    let task_id = if let Some(err) = faucet_resp.error {
        bail!("Faucet request was unsuccessful. Error is {err:?}")
    } else {
        faucet_resp.task
    };

    println!("Faucet request task id: {task_id}");

    // let json_body = json![{
    //     "GetBatchSendStatusRequest": {
    //         "task_id": &task_id
    //     }
    // }];

    // let mut coin_id = "".to_string();

    // // wait for the faucet to finish the batch of token requests
    // loop {
    //     let resp = client
    //         .get("https://faucet.testnet.sui.io/v1/status")
    //         .header("Content-Type", "application/json")
    //         .json(&json_body)
    //         .send()
    //         .await?;
    //     let text = resp.text().await?;
    //     if text.contains("SUCCEEDED") {
    //         let resp_json: serde_json::Value = serde_json::from_str(&text).unwrap();

    //         coin_id = <&str>::clone(
    //             &resp_json
    //                 .pointer("/status/transferred_gas_objects/sent/0/id")
    //                 .unwrap()
    //                 .as_str()
    //                 .unwrap(),
    //         )
    //         .to_string();

    //         break;
    //     } else {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    // }

    // // wait until the fullnode has the coin object, and check if it has the same owner
    // loop {
    //     let owner = sui_client
    //         .read_api()
    //         .get_object_with_options(
    //             ObjectID::from_str(&coin_id)?,
    //             SuiObjectDataOptions::new().with_owner(),
    //         )
    //         .await?;

    //     if owner.owner().is_some() {
    //         let owner_address = owner.owner().unwrap().get_owner_address()?;
    //         if owner_address == address {
    //             break;
    //         }
    //     } else {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    // }
    Ok(())
}
#[test]
async fn test_batch_create_account(){
    let address_json_file="./test_address_data.json";
    let key_store_file="./test_key_store.keystore";
    let aliases_file: &str="./test_key_store.aliases";
    let account=batch_create_account(100,address_json_file,key_store_file).unwrap();
    assert_eq!(account.len(),100);
    fs::remove_file(address_json_file).unwrap();
    fs::remove_file(key_store_file).unwrap();
    fs::remove_file(aliases_file).unwrap();
}

pub async fn fetch_coin(
    sui: &SuiClient,
    sender: &SuiAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::sui::SUI".to_string();
    let coins_stream = sui
        .coin_read_api()
        .get_coins_stream(*sender, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();
    let coin = coins.next().await;
    Ok(coin)
}

#[test]
async fn test_send_tranfer_tx(){
    let address_json_file="./test_address_data.json";
    let key_store_file="./test_key_store.keystore";
    let accounts=batch_create_account(10,address_json_file,key_store_file).unwrap();
    let client = SuiClientBuilder::default().build_testnet().await.unwrap();
    let sender: SuiAddress=accounts[0].account.parse().unwrap();

    let receiver: SuiAddress=accounts[1].account.parse().unwrap();
    request_tokens_from_faucet(sender, &client).await.unwrap();
    tokio::time::sleep(Duration::from_secs(100)).await;
    let coin=fetch_coin(&client, &sender).await.unwrap().unwrap();

    let resopnse=send_tranfer_tx(&client, key_store_file,sender, receiver, 5_0_000_000, coin,30000000).await.unwrap();
    println!("{:?}",resopnse);

}