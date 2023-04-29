use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MintData {
    pub token_id: i32,
    pub token_address: String,
    pub name: String,
}
