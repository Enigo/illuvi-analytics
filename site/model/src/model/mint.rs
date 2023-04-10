use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MintData {
    pub transaction_id: i32,
    pub wallet: String,
    pub token_id: i32,
    pub token_address: String,
    pub minted_on: NaiveDateTime,
    pub name: String,
}
