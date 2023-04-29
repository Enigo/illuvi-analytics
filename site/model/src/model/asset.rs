use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetData {
    pub token_id: i32,
    pub token_address: String,
    pub current_owner: String,
    pub last_owner_change: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionData {
    pub id: Option<i32>,
    pub wallet_from: String,
    pub wallet_to: String,
    pub event: String,
    pub updated_on: NaiveDateTime,
    pub price: Option<Price>,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub price: f32,
    pub currency: String,
}

#[derive(Serialize, Deserialize)]
pub struct LandAssetData {
    pub asset_data: AssetData,
    pub transaction_data: Vec<TransactionData>,
    pub name: String,
    pub tier: i32,
    pub solon: i32,
    pub carbon: i32,
    pub crypton: i32,
    pub silicon: i32,
    pub hydrogen: i32,
    pub hyperion: i32,
    pub landmark: String,
}
