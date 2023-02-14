use crate::api_reader::utils;
use crate::db::assets_handler::AssetSaver;
use crate::env_utils;

const ASSETS_URL: &str =
    "https://api.x.immutable.com/v1/assets?collection=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

pub async fn read_assets() {
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        utils::read_with_cursor_as(ASSETS_URL, &AssetSaver).await
    }
}
