use crate::model::price::Price;
use crate::model::trade::SingleTrade;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct VitalsData {
    pub total_assets: i64,
    pub unique_holders: i64,
    pub trades_volume: Vec<Price>,
    pub last_trades: Vec<SingleTrade>,
    pub data_by_attribute: BTreeMap<String, AttributeData>,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeData {
    pub floor: Vec<VitalsDataFloor>,
    pub minted_burnt: TotalMintedBurnt,
    pub active_orders: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VitalsDataFloor {
    pub token_id: i32,
    pub name: String,
    pub image_url: String,
    pub price: Price,
    pub usd_price: Price,
}

#[derive(Serialize, Deserialize)]
pub struct TotalMintedBurnt {
    pub total_minted: i64,
    pub total_burnt: i64,
}
