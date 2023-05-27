use crate::api_reader::immutablex::utils;
use crate::db::immutablex::transfers_handler::TransferSaver;
use crate::model::immutablex::transfer::Transfer;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const TRANSFERS_URL: &str = "https://api.x.immutable.com/v1/transfers?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&order_by=created_at&direction=asc";

pub async fn read_transfers(pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("TRANSFERS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Transfer>(
            pool,
            TRANSFERS_URL,
            "min_timestamp",
            &TransferSaver,
        )
        .await;
    }
}
