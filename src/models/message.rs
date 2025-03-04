use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: Option<u64>,
}
