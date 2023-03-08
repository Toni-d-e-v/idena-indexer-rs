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
use crate::schema::blocks::dsl::blocks;
pub fn getBlockByHash(conn: &mut PgConnection, hash: String) -> BlockDB {
    use crate::schema::blocks::dsl::{blocks, hash as block_hash};
    let result = blocks
        .filter(block_hash.eq(hash.clone()))
        .first::<BlockDB>(conn)
        .expect(&("Error loading block by hash ".to_owned() + &hash));
    println!("Block Found: {}", result.hash);
    result
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
        transactions: if block["transactions"].is_array() {
            let mut transactions = String::from("");
            for transaction in block["transactions"].as_array().unwrap() {
                transactions.push_str(&transaction.as_str().unwrap().to_string());
                if transaction != block["transactions"].as_array().unwrap().last().unwrap() {
                    transactions.push_str(",");
                }
            }
            transactions
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



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  


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
            }
            sleep(Duration::from_secs(1)).await;

        }
        
        
    });
    task::spawn(async move{
        let mut db = establish_connection();
        
        loop {
            let mut apiloop = IdenaAPI::new("idena-restricted-node-key", "https://restricted.idena.io");
            let mut lastest = getLastBlock(&mut db);
            // this is thread to sync all blocks from lastest to 0 if block is not synced
            let mut height = lastest.height;
            for i in 0..height {
                let doesExist1 = doesExist(&mut db, (height - i).try_into().unwrap());
                if !doesExist1 {
                    sync_block(&mut db,apiloop.clone(), (height - i).try_into().unwrap()).await;
                }
            }
            
            

          }
    });

    

    // wait for ctrl    
    tokio::signal::ctrl_c().await?;

    // rocket
    Ok(())


}




