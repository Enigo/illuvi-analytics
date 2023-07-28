use crate::model::price::Price;
use crate::model::trade::SingleTrade;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct VitalsData {
    pub total_assets: i64,
    pub unique_holders: i64,
    pub floor_by_attribute: BTreeMap<String, Vec<VitalsDataFloor>>,
    pub trades_volume: Vec<Price>,
    pub last_trades: Vec<SingleTrade>,
    pub minted_burnt_by_attribute: BTreeMap<String, Vec<TotalMintedBurnt>>,
}

#[derive(Serialize, Deserialize)]
pub struct VitalsDataFloor {
    pub token_id: i32,
    pub name: String,
    pub price: Price,
}

#[derive(Serialize, Deserialize)]
pub struct TotalMintedBurnt {
    pub total_minted: i64,
    pub total_burnt: i64,
}
