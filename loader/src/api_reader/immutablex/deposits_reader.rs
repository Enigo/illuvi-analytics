use crate::api_reader::immutablex::utils;
use crate::db::immutablex::deposits_handler::DepositSaver;
use crate::model::immutablex::deposit::Deposit;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const DEPOSITS_URL: &str =
    "https://api.x.immutable.com/v1/deposits?page_size=200&order_by=created_at&token_address=";

pub async fn read_deposits(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("DEPOSITS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Deposit>(
            pool,
            format!("{}{}", DEPOSITS_URL, token_address).as_str(),
            "min_timestamp",
            token_address,
            &DepositSaver,
        )
        .await;
    }
}
