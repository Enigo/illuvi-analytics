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
    pub usd_price: Option<Price>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetData {
    pub land: Option<LandAssetData>,
    pub d1sk: Option<D1skAssetData>,
    pub accessories: Option<AccessoriesAssetData>,
    pub illuvitar: Option<IlluvitarAssetData>,
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
    pub content: Vec<AssetContentData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AssetContentData {
    pub token_id: i32,
    pub token_address: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AccessoriesAssetData {
    pub common_asset_data: CommonAssetData,
    pub tier: String,
    pub stage: String,
    pub slot: String,
    pub source_token_address: String,
    pub source_disk_type: String,
    pub source_disk_id: i32,
    pub multiplier: String,
    pub illuvitar: Option<AssetContentData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct IlluvitarAssetData {
    pub common_asset_data: CommonAssetData,
    pub set: String,
    pub line: String,
    pub tier: String,
    pub wave: String,
    pub stage: String,
    pub class: String,
    pub affinity: String,
    pub expression: String,
    pub total_power: i32,
    pub source_token_address: String,
    pub source_disk_type: String,
    pub source_disk_id: i32,
    pub origin_illuvitar_id: Option<i32>,
    pub accessorised_illuvitar_id: Option<i32>,
    pub accessories: Vec<AssetContentData>,
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
