use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MintData {
    pub total: i64,
    pub mints: Vec<Mint>,
}

#[derive(Serialize, Deserialize)]
pub struct Mint {
    pub token_id: i32,
    pub token_address: String,
    pub name: String,
    pub image_url: String,
}
