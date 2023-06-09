use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub offset: u64,
    pub timestamp: u64,
    pub id: u128,
    #[serde(skip)]
    pub length: u32,
    #[serde_as(as = "Base64")]
    pub payload: Vec<u8>,
}

impl Message {
    pub fn get_size_bytes(&self) -> u32 {
        // Offset + Timestamp + ID + Length + Payload
        8 + 8 + 16 + 4 + self.payload.len() as u32
    }
}
