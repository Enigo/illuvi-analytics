use crate::api_reader::immutablex::utils;
use crate::db::immutablex::collection_handler;
use crate::model::immutablex::collection::Collection;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const COLLECTION_URL: &str =
    "https://api.x.immutable.com/v1/collections?page_size=200&keyword=illuv";

pub async fn read_collections(pool: &Pool<Postgres>) -> Vec<String> {
    if env_utils::as_parsed::<bool>("COLLECTIONS_ENABLED") {
        let collections =
            utils::fetch_all_api_responses_with_cursor::<Collection>(COLLECTION_URL).await;

        for collection in collections {
            collection_handler::create_one(&collection, &pool).await;
        }
    }

    collection_handler::fetch_all_enabled_collections(&pool).await
}
