use crate::api_reader::immutablex::utils;
use crate::db::immutablex::assets_handler::AssetSaver;
use crate::model::immutablex::asset::Asset;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const ASSETS_URL: &str = "https://api.x.immutable.com/v1/assets?page_size=200&collection=";

pub async fn read_assets(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Asset>(
            pool,
            format!("{}{}", ASSETS_URL, token_address).as_str(),
            "updated_min_timestamp",
            token_address,
            &AssetSaver,
        )
        .await;
    }
}
