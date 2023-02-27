use std::fmt;

#[derive(Debug)]
pub enum IdenaError {
    /// Error sent from the node. Contains the json string recieved.
    NodeError(String),
    /// Error through trying to request.
    RequestError(reqwest::Error),
}

impl fmt::Display for IdenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeError(node_response_json) => write!(f, "Node returned error: {}", node_response_json),
            Self::RequestError(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for IdenaError {}

impl From<reqwest::Error> for IdenaError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err)
    }
}