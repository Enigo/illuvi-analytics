use crate::model::price::Price;
use crate::model::trade::SingleTrade;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VitalsData {
    pub total_assets: i64,
    pub unique_holders: i64,
    pub floor: Vec<VitalsDataFloor>,
    pub trades_volume: Vec<Price>,
    pub last_trades: Vec<SingleTrade>,
}

#[derive(Serialize, Deserialize)]
pub struct VitalsDataFloor {
    pub tier: i32,
    pub token_id: i32,
    pub name: String,
    pub price: Price,
}
