use diesel::prelude::*;
use crate::schema::blocks;


#[derive(Queryable)]

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