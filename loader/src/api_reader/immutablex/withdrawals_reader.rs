use crate::api_reader::immutablex::utils;
use crate::db::immutablex::withdrawals_handler::WithdrawalSaver;
use crate::model::immutablex::withdrawal::Withdrawal;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const WITHDRAWALS_URL: &str =
    "https://api.x.immutable.com/v1/withdrawals?page_size=200&order_by=created_at&token_address=";

pub async fn read_withdrawals(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("WITHDRAWALS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Withdrawal>(
            pool,
            format!("{}{}", WITHDRAWALS_URL, token_address).as_str(),
            "min_timestamp",
            token_address,
            &WithdrawalSaver,
        )
        .await;
    }
}
