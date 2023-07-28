use crate::api_reader::immutablex::utils;
use crate::db::immutablex::transfers_handler::TransferSaver;
use crate::model::immutablex::transfer::Transfer;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const TRANSFERS_URL: &str =
    "https://api.x.immutable.com/v1/transfers?page_size=200&order_by=created_at&token_address=";

pub async fn read_transfers(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("TRANSFERS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Transfer>(
            pool,
            format!("{}{}", TRANSFERS_URL, token_address).as_str(),
            "min_timestamp",
            token_address,
            &TransferSaver,
        )
        .await;
    }
}
