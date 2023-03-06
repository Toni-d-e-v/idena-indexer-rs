use std::fmt::format;
use tokio::io;
use api::IdenaAPI;
use tokio::io::AsyncWriteExt;
use tokio::task;
use tokio::time::Duration;
// sleep
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::Mutex;
mod block;
use block::Block;
use sled::Db;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    /// open db or create if not exist
    static ref DB: Db = sled::open("db").unwrap();
}



async fn set_db(key: &str, value: &str) {

    DB.insert(key, value).unwrap();
}

async fn get_db(key: &str) -> String {
    match DB.get(key) {
        Ok(Some(value)) => {
            let value = value.to_vec();
            let value = String::from_utf8(value).unwrap();
            value
        }
        Ok(None) => {
            println!("No value found for key: {}", key);
            String::from("nof")
        }
        Err(e) => {
            println!("Error: {}", e);
            String::from("")
        }
    }
}

async fn lastest_synced_block() -> u64 {
    let value = get_db("lastest_synced_block").await;
    // nof check
    if value == "nof" {
        set_lastest_synced_block(0).await;
        return 0;

    }
    match value.parse::<u64>() {
        Ok(value) => {
            value
        
        },
        Err(e) => {
            
            0
        }

    }

}
async fn set_lastest_synced_block(value: u64) {
    set_db("lastest_synced_block", &value.to_string()).await;
}

// synced-range
async fn synced_range() -> String {
    let value = get_db("synced_range").await;
    if value == "nof" {
        set_synced_range(String::from("0-0")).await;
        return String::from("0-0");
    }
    if value == "" {
        String::from("0-0")
    } else {
        value
    }
}
async fn set_synced_range(value: String) {
    set_db("synced_range", &value).await;
}

async fn sync_block(api: IdenaAPI, height: usize) {
    //let block = api.block_at(height).await.unwrap();
    let block = match api.block_at(height).await {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let string = format!("block-{}", (block["height"].as_u64().unwrap()).to_string());
    let mut block_struct = Block {
        coinbase: block["coinbase"].as_str().unwrap().to_string(),
        flags: match block["flags"].as_str() {
            Some(value) => value.to_string(),
            None => String::from(""),
        },
        hash: block["hash"].as_str().unwrap().to_string(),
        height: block["height"].as_u64().unwrap(),
        identityRoot: block["identityRoot"].as_str().unwrap().to_string(),
        ipfsCid: match block["ipfsCid"].as_str() {
            Some(value) => value.to_string(),
            None => String::from(""),
        },
        isEmpty: block["isEmpty"].as_bool().unwrap(),
        offlineAddress: match block["offlineAddress"].as_str() {
            Some(value) => value.to_string(),
            None => String::from(""),
        },
        parentHash: block["parentHash"].as_str().unwrap().to_string(),
        root: block["root"].as_str().unwrap().to_string(),
        timestamp: block["timestamp"].as_u64().unwrap(),
        transactions: match block["transactions"].as_str() {
            Some(value) => value.to_string(),
            None => String::from(""),
        },
    };
    set_db(&string, &block_struct.to_string()).await;

    
    if block_struct.height > lastest_synced_block().await {
        set_lastest_synced_block(block_struct.height).await;
    } else if block_struct.height < lastest_synced_block().await {
        let mut range = synced_range().await;
        let mut range = range.split("-");
        let mut range = (range.next().unwrap().parse::<u64>().unwrap(), range.next().unwrap().parse::<u64>().unwrap());
        if range.0 == 0 {
            range.0 = block_struct.height;
        }
  

        if block_struct.height < range.0 {
            let new_range = format!("{}-{}", block_struct.height, range.1);
            set_synced_range(new_range).await;
        } else if block_struct.height > range.1 {
            let new_range = format!("{}-{}", range.0, block_struct.height);
            set_synced_range(new_range).await;
        }
    }
}
async fn get_block(height: u64) -> Block {
    let string = format!("block-{}", height.to_string());
    let value = get_db(&string).await;
    let block = Block::from_string(value);
    block
}
// gBlockJson


async fn calulate_next_batch(size: u64) -> (u64, u64) {
    let mut range = synced_range().await;
    let mut range = range.split("-");
    let mut range = (range.next().unwrap().parse::<u64>().unwrap(), range.next().unwrap().parse::<u64>().unwrap());
    // calulate next batch is synced range is 90-100 next batch is 80-100
    let mut next_batch = (range.0 - size, range.1);
    if next_batch.0 < 0 {
        next_batch.0 = 0;
    }
    next_batch
}
async fn behind_range(api: IdenaAPI) -> (u64, u64) {
    let mut range = synced_range().await;
    let mut range = range.split("-");
    let range = (range.next().unwrap().parse::<u64>().unwrap(), range.next().unwrap().parse::<u64>().unwrap());
    let lastest = api.last_block().await.unwrap();
    (range.1, lastest["height"].as_u64().unwrap())
}



//use serde_json::Value;
//use serde_js


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



#[launch]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");

    task::spawn(async move{
        println!("Idena indexer in rust");
        let _response = api.epoch().await.unwrap();
        println!("Epoch: {}", _response);
        // get lastest synced block and sync range and check if exists
        // returns u64
        // check if exists
        match lastest_synced_block().await {
            0 => {
                println!("No lastest synced block found");
            }
            _ => {
                println!("Lastest synced block: {}", lastest_synced_block().await);
            }
        }
        // last_block
        let _response = api.last_block().await.unwrap();
        println!("Last block: {}", _response);
        // index last 10 blocks
        // then get them from db

        let lastest = _response;
        //sync_block(api.clone(), (lastest["height"].as_u64().unwrap() - i).try_into().unwrap()).await;


        println!("Lastest synced block: {}", lastest_synced_block().await);
        // get lastest_synced_block
        let string = format!("block-{}", lastest_synced_block().await.to_string());
        let value = get_db(&string).await;
        println!("Lastest synced block: {}", value);
        // synced_range
        println!("Synced range: {}", synced_range().await);
        // next batch
        let next_batch = calulate_next_batch(10).await;
        println!("Next batch: {}-{}", next_batch.0, next_batch.1);
        // behind range
        println!("Behind range: {}-{}", behind_range(api.clone()).await.0, behind_range(api.clone()).await.1);
        // 2 threads one that syncs old blocks and behind range

        while true {
            // first check if behind range
            let behind_range = behind_range(api.clone()).await;
            if behind_range.0 < behind_range.1 {
                println!("Behind range: {}-{}", behind_range.0, behind_range.1);
                // sync blocks
                for i in behind_range.0..behind_range.1 {
                    println!("Syncing block: {}", i);
                    sync_block(api.clone(),  i.try_into().unwrap()).await;
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
    // wait for threads to finish

    // rocket
    Ok(())


}



// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

