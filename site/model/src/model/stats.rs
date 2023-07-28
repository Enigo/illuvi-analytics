use crate::model::price::Price;
use crate::model::trade::SingleTrade;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct StatsData {
    pub total: StatsDataTotal,
    pub trades_volume: Vec<StatsDataTradesVolume>,
    pub trades_by_status: BTreeMap<String, Vec<StatsDataTotalOrder>>,
    pub most_transferred_token: Vec<StatsDataMostEventForToken>,
    pub most_traded_token: Vec<StatsDataMostEventForToken>,
    pub most_traded_wallet: Vec<StatsDataMostEventForWallet>,
    pub cheapest_and_most_expensive_trades_by_attribute: BTreeMap<String, Vec<SingleTrade>>,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDataTotal {
    pub assets_minted: i64,
    pub assets_burnt: i64,
    pub transfers: i64,
    pub trades: i64,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDataTotalOrder {
    pub count: i64,
    pub buy_currency: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct StatsDataTradesVolume {
    pub total_trades: i64,
    pub total_in_buy_currency: Price,
    pub totals_in_other_currency: Vec<Price>,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDataMostEventForToken {
    pub count: i64,
    pub token_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDataMostEventForWallet {
    pub count: i64,
    pub wallet: String,
}
