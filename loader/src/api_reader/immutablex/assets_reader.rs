use crate::api_reader::immutablex::utils;
use crate::db::immutablex::assets_handler::AssetSaver;
use crate::model::immutablex::asset::Asset;
use crate::utils::env_utils;

const ASSETS_URL: &str =
    "https://api.x.immutable.com/v1/assets?collection=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

pub async fn read_assets() {
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor::<Asset>(
            ASSETS_URL,
            "updated_min_timestamp",
            &AssetSaver,
        )
        .await
    }
}
