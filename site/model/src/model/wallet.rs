use serde::{Deserialize, Serialize};
use crate::model::price::Price;

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub wallet: String,
    pub minted_per_collection_wallet: Vec<TotalPerCollectionData>,
    pub owned_per_collection_wallet: Vec<TotalPerCollectionData>,
    pub money_data: WalletMoneyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalPerCollectionData {
    pub total_per_wallet: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WalletMoneyData {
    pub mint_spend_usd: Price,
    pub total_sell_usd: Price,
    pub total_buy_usd: Price,
    pub total_active_usd: Price,
    pub total_active: i64,
}