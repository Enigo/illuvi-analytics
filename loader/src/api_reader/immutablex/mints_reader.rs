use crate::api_reader::immutablex::utils;
use crate::db::immutablex::mints_handler::MintSaver;
use crate::model::immutablex::mint::Mint;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const MINTS_URL: &str = "https://api.x.immutable.com/v1/mints?page_size=200&token_address=";

pub async fn read_mints(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("MINTS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Mint>(
            pool,
            format!("{}{}", MINTS_URL, token_address).as_str(),
            "min_timestamp",
            token_address,
            &MintSaver,
        )
        .await
    }
}
