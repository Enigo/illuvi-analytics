use crate::model::price::Price;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct SingleTrade {
    pub token_id: i32,
    pub name: String,
    pub image_url: String,
    pub usd_price: Price,
    pub buy_price: Price,
    pub updated_on: NaiveDateTime,
    pub transaction_id: i32,
}
