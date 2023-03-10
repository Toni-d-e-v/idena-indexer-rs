use diesel::prelude::*;
use crate::schema::blocks;
use crate::schema::transactions;


#[derive(Queryable)]
#[derive(Debug)]

pub struct BlockDB {
    pub coinbase: String,
    pub flags: String,
    pub hash: String,
    pub height: i32,
    pub identityRoot: String,
    pub ipfsCid: String,
    pub isEmpty: bool,
    pub offlineAddress: String,
    pub parentHash: String,
    pub root: String,
    pub timestamp: i32,
    pub transactions: String,
}

#[derive(Insertable)]
#[derive(Debug)]

#[diesel(table_name = blocks)]
pub struct NewBlockDB<'a> {
    pub coinbase: &'a str,
    pub flags: &'a str,
    pub hash: &'a str,
    // Int4 no i64
    pub height: &'a i32,
    pub identityroot: &'a str,
    pub ipfscid: &'a str,
    pub isempty: bool,
    pub offlineaddress: &'a str,
    pub parenthash: &'a str,
    pub root: &'a str,
    pub timestamp: &'a i32,
    pub transactions: &'a str,
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct TransactionDB {
    pub epoch: i32,
    pub blockheight: i32,
    pub blockhash: String,
    pub hash: String,
    pub type_: String,
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub tips: String,
    pub maxfee: String,
    pub fee: String,
    pub size: i32,
    pub nonce: i32,
}

#[derive(Insertable)]
#[derive(Debug)]
#[diesel(table_name = transactions)]

pub struct NewTransactionDB<'a> {
    pub epoch: &'a i32,
    pub blockheight: &'a i32,
    pub blockhash: &'a str,
    pub hash_: &'a str,
    pub type_: &'a str,
    pub timestamp_: &'a str,
    pub from_: &'a str,
    pub to_: &'a str,
    pub amount: &'a str,
    pub tips: &'a str,
    pub maxfee: &'a str,
    pub fee: &'a str,
    pub size: &'a i32,
    pub nonce: &'a i32,
}
