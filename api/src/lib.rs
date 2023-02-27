#![warn(rust_2018_idioms)]

mod error;

use serde_json::{json, Value};
use reqwest::Client;
pub use error::IdenaError;

// Macro used to simplify code when requesting.
macro_rules! do_request {
    ($self:expr, $json:tt) => {
        $self.request(
            json!($json)
        ).await
    }
}

/// The main API object.
// clone
#[derive(Clone)]
pub struct IdenaAPI {
    /// The API key for your node.
    api_key: String,
    /// The host URL of your node. Usually http://localhost:9119/ if you are running the internal node of idena-desktop.
    host_url: String,
    /// The client object used to send requests to the node.
    client: Client,
}

impl IdenaAPI {
    pub fn new(api_key: &str, host_url: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            host_url: host_url.to_owned(),

            client: Client::new(),
        }
    }

    #[inline]
    async fn request(&self, payload: Value) -> Result<Value, IdenaError> {
        let response: Value = self.client.post(&self.host_url)
            .json(&payload)
            .send()
            .await? // Wait for it to send, then receive
            .json()  // Parse as hashmap.
            .await?;
        
        // Check for error
        if response["error"] != Value::Null {
            let error_json = response["error"].to_string();
            Err(IdenaError::NodeError(error_json))
        } else {
            Ok(response["result"].clone())
        }
    }

    /// Change the API key.
    pub fn set_api_key(&mut self, new_key: &str) {
        self.api_key = new_key.to_owned();
    }

    /// List all identities (not only validated ones).
    pub async fn identities(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_identities",
        })
    }

    /// Show info about identity for a given address.
    pub async fn identity(&self, address: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_identity",
            "params": [address.to_owned()]
        })
    }

    /// Get the current epoch.
    pub async fn epoch(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_epoch",
        })
    }

    /// Get the interval between ceremonies.
    pub async fn ceremony_intervals(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "dna_ceremonyIntervals",
            "id": 1,
        })
    }

    /// Get this node's address.
    pub async fn address(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "dna_getCoinbaseAddr",
            "id": 1,
        })
    }

    /// Get the balance of an address.
    pub async fn balance(&self, address: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_getBalance".to_owned(),
            "params": [address.to_owned()],
        })
    }

    /// Get information about a transaction.
    pub async fn transaction(&self, trx_hash: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_transaction",
            "params": [trx_hash],
            "id": 1,
        })
    }

    /// Get previous transactions.
    pub async fn transactions(&self, address: &str, count: usize) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_transactions",
            "params": [{ "address": address, "count": count }],
            "id": 1,
        })
    }

    /// Get pending transactions.
    pub async fn pending_transactions(&self, address: &str, count: usize) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_pendingTransactions",
            "params": [{ "address": address, "count": count }],
            "id": 1,
        })
    }

    /// Kill this identity.
    pub async fn kill_identity(&self, address: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "dna_sendTransaction",
            "params": [{ "type": 3, "from": address, "to": address }],
            "id": 1,
        })
    }

    /// Set mining online.
    pub async fn go_online(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "dna_becomeOnline",
            "params": [{}],
            "id": 1,
        })
    }

    /// Set mining offline.
    pub async fn go_offline(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "dna_becomeOffline",
            "params": [{}],
            "id": 1,
        })
    }
    // bcn_lastBlock
    pub async fn last_block(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_lastBlock",
            "id": 1,
        })
    }
    // bcn_blockAt
    pub async fn block_at(&self, height: usize) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_blockAt",
            "params": [height],
            "id": 1,
        })
    }

    pub async fn block_by_hash(&self, hash: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_blockByHash",
            "params": [hash],
            "id": 1,
        })
    }
    // bcn_mempool
    pub async fn mempool(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_mempool",
            "id": 1,
        })
    }


    /// Send DNA from one address to another.
    pub async fn send(&self, from_address: &str, to_address: &str, amount: f64) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_sendTransaction",
            "params": [{"from": from_address, "to": to_address, "amount": amount}],
        })
    }

    /// Get the sync status of the node.
    pub async fn sync_status(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "method": "bcn_syncing",
            "id": 1,
        })
    }

}
