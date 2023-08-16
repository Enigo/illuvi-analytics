use crate::api_reader::api_utils;
use crate::api_reader::immutablex::utils;
use crate::db::immutablex::{assets_handler, assets_handler::AssetSaver};
use crate::model::immutablex::asset::{Asset, TheResult};
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const ASSETS_URL: &str = "https://api.x.immutable.com/v1/assets?page_size=200&collection=";
const ASSET_URL: &str = "https://api.x.immutable.com/v1/assets";

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

pub async fn update_metadata(pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        for pair in assets_handler::fetch_all_assets_with_no_metadata(pool).await {
            let result = api_utils::fetch_single_api_response::<TheResult>(
                format!("{}/{}/{}", ASSET_URL, pair.0, pair.1).as_str(),
            )
            .await;

            if result.is_some() {
                let asset = result.unwrap();
                let metadata = asset.metadata;
                if metadata.is_some() {
                    assets_handler::update_metadata(metadata.unwrap(), &pair.0, &pair.1, pool)
                        .await;
                }
            }
        }
    }
}
