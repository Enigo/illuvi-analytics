use crate::model::price::Price;
use crate::model::transaction::SingleTransaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetData {
    pub land: Option<LandAssetData>,
    pub d1sk: Option<D1skAssetData>,
    pub accessories: Option<AccessoriesAssetData>,
    pub illuvitar: Option<IlluvitarAssetData>,
    pub blueprint: Option<BlueprintAssetData>,
    pub event: Option<EventAssetData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct LandAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
    pub tier: String,
    pub solon: String,
    pub carbon: String,
    pub crypton: String,
    pub silicon: String,
    pub hydrogen: String,
    pub hyperion: String,
    pub landmark: String,
    pub total_discovered_blueprints: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct D1skAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
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
    pub image_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AccessoriesAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
    pub tier: String,
    pub stage: String,
    pub slot: String,
    pub multiplier: String,
    pub d1sk: Option<AssetContentData>,
    pub illuvitar: Option<AssetContentData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct IlluvitarAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
    pub set: String,
    pub line: String,
    pub tier: String,
    pub wave: String,
    pub stage: String,
    pub class: String,
    pub affinity: String,
    pub expression: String,
    pub total_power: i32,
    pub d1sk: Option<AssetContentData>,
    pub origin_illuvitar: Option<AssetContentData>,
    pub accessorised_illuvitar: Option<AssetContentData>,
    pub accessories: Vec<AssetContentData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct BlueprintAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
    pub item_tier: String,
    pub item_type: String,
    pub discovered_by: String,
    pub land: Option<AssetContentData>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EventAssetData {
    pub common_asset_data: CommonAssetData,
    pub common_order_data: Option<CommonOrderData>,
    pub line: String,
    pub promotion: String,
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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct CommonOrderData {
    pub active_orders: i64,
    pub buy_price: Option<Price>,
    pub total_filled_orders: i64,
    pub listed_index: Option<i64>,
    pub last_active_orders: Vec<SingleTransaction>,
    pub last_filled_orders: Vec<SingleTransaction>,
}
