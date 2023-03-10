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

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}



use self::models::{BlockDB, NewBlockDB};
use self::models::{TransactionDB, NewTransactionDB};
use crate::schema::blocks::dsl::blocks;
use crate::schema::transactions::dsl::transactions;

pub fn getBlockByHash(conn: &mut PgConnection, hash: String) -> BlockDB {
    use crate::schema::blocks::dsl::{blocks, hash as block_hash};
    let result = blocks
        .filter(block_hash.eq(hash.clone()))
        .first::<BlockDB>(conn)
        .expect(&("Error loading block by hash ".to_owned() + &hash));
    println!("Block Found: {}", result.hash);
    result
}

pub fn getTxByHash(conn: &mut PgConnection, hash: String) -> TransactionDB {
    use crate::schema::transactions::dsl::{hash_ as tx_hash, transactions};
    let result = transactions
        .filter(tx_hash.eq(hash.clone()))
        .first::<TransactionDB>(conn)
        .expect(&("Error loading transaction by hash ".to_owned() + &hash));
    println!("Transaction Found: {}", result.hash);
    result
}

async fn getTxsByAddress(conn: &mut PgConnection, address: String) -> Vec<String> {
    use crate::schema::transactions::dsl::{from_ as tx_address, transactions};
    let results = transactions
        .filter(tx_address.eq(address.clone()))
        .load::<TransactionDB>(conn)
        .expect(&("Error loading transactions by address ".to_owned() + &address));

   
    let mut tx_hashes: Vec<String> = Vec::new();
    for tx in results {
        tx_hashes.push(tx.hash);
    }
    tx_hashes
 
}


pub fn getBlockByHeight(conn: &mut PgConnection, height1: i32) -> BlockDB {
    use crate::schema::blocks::dsl::{blocks, height as block_height};
    let result = blocks
        .filter(block_height.eq(height1))
        .first::<BlockDB>(conn)
        .expect(&("Error loading block by height ".to_owned() + &height1.to_string()));
    
    println!("Block Found: {}", result.hash);
    result
}

pub fn getLastBlock(conn: &mut PgConnection) -> BlockDB {
    use crate::schema::blocks::dsl::{blocks, height as block_height};
    let result = blocks
        .order(block_height.desc())
        .first::<BlockDB>(conn)
        .expect("Error loading last block");
    
    println!("Block Found: {}", result.hash);
    result
}

pub fn doesExist(conn: &mut PgConnection, height: i32) -> bool {
    use crate::schema::blocks::dsl::{blocks, height as block_height};
    let result = blocks
        .filter(block_height.eq(height))
        .first::<BlockDB>(conn)
        .is_ok();
    result
}

async fn sync_tx ( conn: &mut PgConnection,api: IdenaAPI, hash: String, height: i32, timestamp: String) {
    let tx = match api.transaction(&hash).await {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("Transaction: {:?}", tx);
    let tx_struct = TransactionDB {
        epoch: tx["epoch"].as_u64().unwrap() as i32,
        blockheight: match tx["blockHeight"].as_u64() {
            Some(value) => value as i32,
            None => height,
        },
        blockhash: tx["blockHash"].as_str().unwrap().to_string(),
        hash: tx["hash"].as_str().unwrap().to_string(),
        type_: tx["type"].as_str().unwrap().to_string(),
        // time is string
        timestamp: match tx["timestamp"].as_str() {
            Some(value) => value.to_string(),
            None => "0".to_string(),
        },
        from: tx["from"].as_str().unwrap().to_string(),
        to: match tx["to"].as_str() {
            Some(value) => value.to_string(),
            None => "0".to_string(),
        },
        amount: tx["amount"].as_str().unwrap().to_string(),
        tips: tx["tips"].as_str().unwrap().to_string(),
        maxfee: tx["maxFee"].as_str().unwrap().to_string(),
        fee: match tx["fee"].as_str() {
            Some(value) => value.to_string(),
            None => "0".to_string(),
        },
        size: match tx["size"].as_u64() {
            Some(value) => value as i32,
            None => 0,
        },
        nonce: tx["nonce"].as_u64().unwrap() as i32,
    };
    let tx_db = NewTransactionDB {
        epoch: &(tx_struct.epoch as i32),
        blockheight: &(tx_struct.blockheight as i32),
        blockhash: &tx_struct.blockhash,
        hash_: &tx_struct.hash,
        type_: &tx_struct.type_,
        timestamp_: &tx_struct.timestamp,
        from_: &tx_struct.from,
        to_: &tx_struct.to,
        amount: &tx_struct.amount,
        tips: &tx_struct.tips,
        maxfee: &tx_struct.maxfee,
        fee: &tx_struct.fee,
        size: &(tx_struct.size as i32),
        nonce: &(tx_struct.nonce as i32),
    };
    let _result =  diesel::insert_into(transactions)
        .values(&tx_db)
        .execute(conn);
}

async fn sync_block(conn: &mut PgConnection,api: IdenaAPI, height: usize) {
    

    //let block = api.block_at(height).await.unwrap();
    let block = match api.block_at(height).await {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let string = format!("block-{}", (block["height"].as_u64().unwrap()).to_string());
    let mut block_struct = BlockDB {
        coinbase: block["coinbase"].as_str().unwrap().to_string(),
        flags: match block["flags"].as_str() {
            Some(value) => value.to_string(),
            None => String::from(""),
        },
        hash: block["hash"].as_str().unwrap().to_string(),
        height: block["height"].as_u64().unwrap() as i32,
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
        timestamp: block["timestamp"].as_u64().unwrap() as i32,
        transactions: if block["transactions"].is_array() {
            let mut transactions_ar = String::from("");
            for transaction in block["transactions"].as_array().unwrap() {
                transactions_ar.push_str(&transaction.as_str().unwrap().to_string());
                if transaction != block["transactions"].as_array().unwrap().last().unwrap() {
                    transactions_ar.push_str(",");
                }
                // sync
                sync_tx(conn,api.clone(),transaction.as_str().unwrap().to_string(),block["height"].as_u64().unwrap() as i32,block["timestamp"].as_str().unwrap().to_string()).await;

            }
            transactions_ar
        } else {
            String::from("")
        },
    };


    let block_db = NewBlockDB {
        coinbase: &block_struct.coinbase,
        flags: &block_struct.flags,
        hash: &block_struct.hash,
        height: &(block_struct.height as i32),
        identityroot: &block_struct.identityRoot,
        ipfscid: &block_struct.ipfsCid,
        isempty: block_struct.isEmpty,
        offlineaddress: &block_struct.offlineAddress,
        parenthash: &block_struct.parentHash,
        root: &block_struct.root,
        timestamp: &(block_struct.timestamp as i32),
        transactions: &block_struct.transactions,
        
    };
    let _result =  diesel::insert_into(blocks)

        .values(&block_db)
        .execute(conn);
        
        
    println!("Block synced: {}", block_struct.hash);

}




use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::{HttpResponse};

fn fix_string(mut string: String) -> String {
    string.replace("BlockDB ", "").replace("", "");
    string = string.replace("\n", "");
    string = string.replace("coinbase", "\"coinbase\"");
    string = string.replace("flags", "\"flags\"");
    string = string.replace("hash", "\"hash\"");
    string = string.replace("height", "\"height\"");
    string = string.replace("identityRoot", "\"identityRoot\"");
    string = string.replace("ipfsCid", "\"ipfsCid\"");
    string = string.replace("isEmpty", "\"isEmpty\"");
    string = string.replace("parentHash", "\"parentHash\"");
    string = string.replace("root", "\"root\"");
    string = string.replace("timestamp", "\"timestamp\"");
    string = string.replace("transactions", "\"transactions\"");
    string = string.replace("offlineAddress", "\"offlineAddress\"");
    string = string.replace("BlockDB", "");
    string 
}
// TransactionDB { epoch: 103, blockheight: 5707712, blockhash: "0x459ea6cbaa13f0b77026260e5042bca5f8f4980a7dbe7e8da53c875e18b67bfd", hash: "0x99b9b804e1a7f5aa9f381f2fb76a9af36e7c36226f62129565a5ea2a72e13e0d", type_: "replenishStake", timestamp: "0", from: "0x09ed45c08be88258e1dd59d3f7d0718e3e867588", to: "0x09ed45c08be88258e1dd59d3f7d0718e3e867588", amount: "10.1372563540503", tips: "0", maxfee: "0.00556586260868", fee: "0", size: 0, nonce: 131 }
fn fix_string_tx(mut string: String) -> String {
    string.replace("TransactionDB ", "").replace("", "");
    string = string.replace("\n", "");
    string = string.replace("epoch", "\"epoch\"");
    string = string.replace("blockheight", "\"block_height\"");
    string = string.replace("blockhash", "\"block_h\"");
    string = string.replace("hash", "\"hash\"");
    string = string.replace("type_", "\"type\"");
    string = string.replace("timestamp", "\"timestamp\"");
    string = string.replace("from", "\"from\"");
    string = string.replace("to", "\"to\"");
    string = string.replace("amount", "\"amount\"");
    string = string.replace("tips", "\"tips\"");
    string = string.replace("maxfee", "\"maxf_ee\"");
    string = string.replace("fee", "\"fee\"");
    string = string.replace("size", "\"size\"");
    string = string.replace("nonce", "\"nonce\"");
    string = string.replace("TransactionDB", "");
    string 

}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Idena rust indexer")
}
#[get("/block/{hash_block}")]
async fn block_api(path: web::Path<(String,)>) -> impl Responder {
    let mut db = establish_connection();
    let blockapi = getBlockByHash(&mut db, path.0.clone());
    let mut string = format!("{:?}", blockapi);
    let string = fix_string(string);

    HttpResponse::Ok().body(format!("{}", string).replace("BlockDB ", "").replace("", ""))
}
#[get("/block/height/{height}")]
async fn block_api_height(path: web::Path<(i32,)>) -> impl Responder {
    let mut db = establish_connection();
    let blockapi = getBlockByHeight(&mut db, path.0.clone());
    let mut string = format!("{:?}", blockapi);
    let string = fix_string(string);

    HttpResponse::Ok().body(format!("{}", string).replace("BlockDB ", "").replace("", ""))
}
#[get("/lastblock")]
async fn last_block_api() -> impl Responder {
    let mut db = establish_connection();
    let blockapi = getLastBlock(&mut db);
    let mut string = format!("{:?}", blockapi);
    let string = fix_string(string);

    HttpResponse::Ok().body(format!("{}", string).replace("BlockDB ", "").replace("", ""))
}

#[get("/last100blocks")]
async fn last_100_blocks_api() -> impl Responder {
    let mut db = establish_connection();
    let mut json = String::from("[");
    let mut lastest = getLastBlock(&mut db);
    for i in 0..100 {
        
        let blockapi = getBlockByHeight(&mut db, lastest.height - i);

        let mut string = format!("{:?}", blockapi);
        json.push_str(&format!("{}", string).replace("BlockDB ", "").replace("", ""));
        // replace coinbase with "coinbase" and so on
  

        if i != 99 {
            json.push_str(",");
        }
    }
    json.push_str("]");
    // remove any whitespace and stuff 
    json = json.replace(" ", "");
    // new line
    json = json.replace("\n", "");
    json = json.replace("coinbase", "\"coinbase\"");
    json = json.replace("flags", "\"flags\"");
    json = json.replace("hash", "\"hash\"");
    json = json.replace("height", "\"height\"");
    json = json.replace("identityRoot", "\"identityRoot\"");
    json = json.replace("ipfsCid", "\"ipfsCid\"");
    json = json.replace("isEmpty", "\"isEmpty\"");
    json = json.replace("offlineAddress", "\"offlineAddress\"");
    json = json.replace("parentHash", "\"parentHash\"");
    json = json.replace("root", "\"root\"");
    json = json.replace("timestamp", "\"timestamp\"");
    json = json.replace("transactions", "\"transactions\"");
    HttpResponse::Ok().body(json)

}



#[get("/tx/{hash_tx}")]
async fn tx_api(path: web::Path<(String,)>) -> impl Responder {
    let mut db = establish_connection();
    let txapi = getTxByHash(&mut db, path.0.clone());
    let mut string = format!("{:?}", txapi);
    let string = fix_string_tx(string);

    HttpResponse::Ok().body(format!("{}", string).replace("TxDB ", "").replace("", ""))
}
#[get("/account/{address}")]
async fn account_api(path: web::Path<(String,)>) -> impl Responder {
    let mut idenaapi = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");
    let mut balance = idenaapi.balance(&path.0.clone()).await.unwrap();
    let mut db = establish_connection();
    let mut txs = getTxsByAddress(&mut db, path.0.clone().to_string()).await;
    // getTxsByAddress -> vec<String> 
    HttpResponse::Ok().body(format!("{{\"address\": \"{}\", \"balance\": {}, \"txs\": {:?}}}", path.0.clone(), balance, txs))
    

}

// All calls



use actix_web::middleware::DefaultHeaders;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  


    task::spawn(async move{
        let mut api = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");

        println!("Idena indexer in rust");
        let _response = api.epoch().await.unwrap();
        println!("Epoch: {}", _response);

        let _response = api.last_block().await.unwrap();
        println!("Last block: {}", _response);
        // index last 10 blocks
        // then get them from db

        let lastest = _response;
        //sync_block(api.clone(), (lastest["height"].as_u64().unwrap() - i).try_into().unwrap()).await;
        let mut db = establish_connection();
        
        // sync lastest block test
        sync_block(&mut db,api.clone(), (lastest["height"].as_u64().unwrap()).try_into().unwrap()).await;
        // get lastest block test
        let block = getBlockByHash(&mut db, (lastest["hash"].as_str().unwrap()).to_string());
        let block = getBlockByHeight(&mut db, (lastest["height"].as_u64().unwrap()).try_into().unwrap());
        let mut lastest_height = 0;
        println!("Lastest block: {}", block.hash);
        loop {
            let mut apiloop = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");

            let _response = apiloop.last_block().await.unwrap();
            let height = _response["height"].as_u64().unwrap();
            if height > lastest_height {
                lastest_height = height;
                sync_block(&mut db,api.clone(), height.try_into().unwrap()).await;
                println!("Lastest block: {}, height: {}", _response["hash"].as_str().unwrap(), height);
            } else {
                println!("No new blocks");
                // check for last 100 blocks
                for i in 0..100 {
                    let doesExist1 = doesExist(&mut db, (height - i).try_into().unwrap());
                    if !doesExist1 {
                        sync_block(&mut db,apiloop.clone(), (height - i).try_into().unwrap()).await;
                    }
                }
            }
            sleep(Duration::from_secs(1)).await;

        }
        
        
    });
    // task::spawn(async move{
    //     let mut db = establish_connection();
        
    //     loop {
    //         let mut apiloop = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");
    //         let mut lastest = getLastBlock(&mut db);
    //         // this is thread to sync all blocks from lastest to 0 if block is not synced
    //         let mut height = lastest.height;
    //         for i in 0..height {
    //             let doesExist1 = doesExist(&mut db, (height - i).try_into().unwrap());
    //             if !doesExist1 {
    //                 sync_block(&mut db,apiloop.clone(), (height - i).try_into().unwrap()).await;
    //             }
    //         }
    //       }
    // });
    // Enable if you want to sync all blockchain from the lastest block to the 0
    



    // wait for ctrl    
    HttpServer::new(|| {
        App::new()
            // Access-Control-Allow-Origin
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))

            .service(index)
            .service(block_api)
            .service(block_api_height)
            .service(last_block_api)
            .service(last_100_blocks_api)
            .service(tx_api)
            .service(account_api)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}


