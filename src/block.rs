

// public
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
        format!("{{\"coinbase\":\"{}\",\"flags\":\"{}\",\"hash\":\"{}\",\"height\":{},\"identityRoot\":\"{}\",\"ipfsCid\":\"{}\",\"isEmpty\":{},\"offlineAddress\":\"{}\",\"parentHash\":\"{}\",\"root\":\"{}\",\"timestamp\":{},\"transactions\":\"{}\"}}", self.coinbase, self.flags, self.hash, self.height, self.identityRoot, self.ipfsCid, self.isEmpty, self.offlineAddress, self.parentHash, self.root, self.timestamp, self.transactions)
    }
    pub fn from_string(&mut self, string: String) {
        let json: serde_json::Value = serde_json::from_str(&string).unwrap();
        self.coinbase = json["coinbase"].as_str().unwrap().to_string();
        self.flags = json["flags"].as_str().unwrap().to_string();
        self.hash = json["hash"].as_str().unwrap().to_string();
        self.height = json["height"].as_u64().unwrap();
        self.identityRoot = json["identityRoot"].as_str().unwrap().to_string();
        self.ipfsCid = json["ipfsCid"].as_str().unwrap().to_string();
        self.isEmpty = json["isEmpty"].as_bool().unwrap();
        self.offlineAddress = json["offlineAddress"].as_str().unwrap().to_string();
        self.parentHash = json["parentHash"].as_str().unwrap().to_string();
        self.root = json["root"].as_str().unwrap().to_string();
        self.timestamp = json["timestamp"].as_u64().unwrap();
        self.transactions = json["transactions"].as_str().unwrap().to_string();
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