

// public

#[derive(Debug, Clone)]
pub struct Block {
    pub coinbase: String,
    pub flags: String,
    pub hash: String,
    pub height: u64,
    pub identityRoot: String,
    pub ipfsCid: String,
    pub isEmpty: bool,
    pub offlineAddress: String,
    pub parentHash: String,
    pub root: String,
    pub timestamp: u64,
    pub transactions: String,
}



impl Block {


    pub fn to_string(&self) -> String {
        // formart without json
        format!("{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}", self.coinbase, self.flags, self.hash, self.height, self.identityRoot, self.ipfsCid, self.isEmpty, self.offlineAddress, self.parentHash, self.root, self.timestamp, self.transactions)
    }
    

    pub fn default() -> Block {
        Block {
            coinbase: String::from(""),
            flags: String::from(""),
            hash: String::from(""),
            height: 0,
            identityRoot: String::from(""),
            ipfsCid: String::from(""),
            isEmpty: false,
            offlineAddress: String::from(""),
            parentHash: String::from(""),
            root: String::from(""),
            timestamp: 0,
            transactions: String::from(""),
        }
    }
    pub fn from_string(block: String) -> Block {
        let mut block = block.split(".");
        let coinbase = block.next().unwrap().to_string();
        let flags = block.next().unwrap().to_string();
        let hash = block.next().unwrap().to_string();
        let height = block.next().unwrap().parse::<u64>().unwrap();
        let identityRoot = block.next().unwrap().to_string();
        let ipfsCid = block.next().unwrap().to_string();
        let isEmpty = block.next().unwrap().parse::<bool>().unwrap();
        let offlineAddress = block.next().unwrap().to_string();
        let parentHash = block.next().unwrap().to_string();
        let root = block.next().unwrap().to_string();
        let timestamp = block.next().unwrap().parse::<u64>().unwrap();
        let transactions = block.next().unwrap().to_string();
        Block {
            coinbase,
            flags,
            hash,
            height,
            identityRoot,
            ipfsCid,
            isEmpty,
            offlineAddress,
            parentHash,
            root,
            timestamp,
            transactions,
        }
       


    }

    
}



// let mut block_struct = Block {
//     coinbase: block["coinbase"].as_str().unwrap().to_string(),
//     flags: match block["flags"].as_str() {
//         Some(value) => value.to_string(),
//         None => String::from(""),
//     },
//     hash: block["hash"].as_str().unwrap().to_string(),
//     height: block["height"].as_u64().unwrap(),
//     identityRoot: block["identityRoot"].as_str().unwrap().to_string(),
//     ipfsCid: match block["ipfsCid"].as_str() {
//         Some(value) => value.to_string(),
//         None => String::from(""),
//     },
//     isEmpty: block["isEmpty"].as_bool().unwrap(),
//     offlineAddress: match block["offlineAddress"].as_str() {
//         Some(value) => value.to_string(),
//         None => String::from(""),
//     },
//     parentHash: block["parentHash"].as_str().unwrap().to_string(),
//     root: block["root"].as_str().unwrap().to_string(),
//     timestamp: block["timestamp"].as_u64().unwrap(),
//     transactions: match block["transactions"].as_str() {
//         Some(value) => value.to_string(),
//         None => String::from(""),
//     },
// };