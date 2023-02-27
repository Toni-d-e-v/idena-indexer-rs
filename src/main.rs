use std::fmt::format;

use api::IdenaAPI;
//     let api = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");
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
            String::from("")
        }
        Err(e) => {
            println!("Error: {}", e);
            String::from("")
        }
    }
}

async fn lastest_synced_block() -> u64 {
    let value = get_db("lastest_synced_block").await;
    if value == "" {
        0
    } else {
        value.parse::<u64>().unwrap()
    }
}
async fn set_lastest_synced_block(value: u64) {
    set_db("lastest_synced_block", &value.to_string()).await;
}

// synced-range
async fn synced_range() -> String {
    let value = get_db("synced_range").await;
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
    println!("Block: {}", block);
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
    let block = Block::from_string(&value);
    block
}



#[tokio::main]
async fn main() {
    println!("Idena indexer in rust");
    let mut api = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");
    let _response = api.epoch().await.unwrap();
    println!("Epoch: {}", _response);
    // last_block
    let _response = api.last_block().await.unwrap();
    println!("Last block: {}", _response);
    // index last 10 blocks
    // then get them from db

    let lastest = _response;
    //sync_block(api.clone(), (lastest["height"].as_u64().unwrap() - i).try_into().unwrap()).await;

    // get from lastest to 0    
    // for i in 0..lastest["height"].as_u64().unwrap() {
    //    let height = (lastest["height"].as_u64().unwrap() - i).try_into().unwrap();
    //    sync_block(api.clone(), height).await;
    //    println!("Synced block: {}", height);
    // }
    // lastest_synced_block
    println!("Lastest synced block: {}", lastest_synced_block().await);
    // get lastest_synced_block
    let string = format!("block-{}", lastest_synced_block().await.to_string());
    let value = get_db(&string).await;
    println!("Lastest synced block: {}", value);
    // synced_range
    println!("Synced range: {}", synced_range().await);




    
}