use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToMainQueRequest {
    pub que_id: u8,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToMainQueResponse {
    pub que_item_key: String,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromResponseQueRequest {
    pub que_id: u8,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromResponseQueResponse {
    pub message: String,
    pub success: bool,
}