use crate::model::asset::AssetContentData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchData {
    pub asset_content_data: Vec<AssetContentData>,
}
