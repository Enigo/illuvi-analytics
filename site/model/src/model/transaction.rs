use crate::model::asset::AssetContentData;
use crate::model::price::Price;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct SingleTransaction {
    pub token_id: i32,
    pub name: String,
    pub image_url: String,
    pub usd_price: Price,
    pub buy_price: Price,
    pub updated_on: NaiveDateTime,
    pub transaction_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EventData {
    pub total: i64,
    pub transactions: Vec<TransactionData>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionData {
    pub id: Option<i32>,
    pub wallet_from: String,
    pub wallet_to: String,
    pub event: String,
    pub updated_on: NaiveDateTime,
    pub price: Option<Price>,
    pub usd_price: Option<Price>,
    pub asset_content: Option<AssetContentData>,
}
