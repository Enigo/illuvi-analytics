use crate::model::price::Price;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
pub struct AssetData {
    pub land: Option<LandAssetData>,
    pub d1sk: Option<D1skAssetData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct LandAssetData {
    pub common_asset_data: CommonAssetData,
    pub tier: String,
    pub solon: String,
    pub carbon: String,
    pub crypton: String,
    pub silicon: String,
    pub hydrogen: String,
    pub hyperion: String,
    pub landmark: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct D1skAssetData {
    pub common_asset_data: CommonAssetData,
    pub alpha: bool,
    pub wave: String,
    pub set: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct CommonAssetData {
    pub token_id: i32,
    pub token_address: String,
    pub current_owner: String,
    pub burned: bool,
    pub name: String,
    pub image_url: String,
}
