use crate::api_reader::api_utils;
use crate::db::transfers_handler::TransferSaver;
use crate::model::transfer::Transfer;
use crate::utils::env_utils;

const TRANSFERS_URL: &str = "https://api.x.immutable.com/v1/transfers?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&order_by=created_at&direction=asc";

pub async fn read_transfers() {
    if env_utils::as_parsed::<bool>("TRANSFERS_ENABLED") {
        api_utils::fetch_and_persist_all_api_responses_with_cursor::<Transfer>(
            TRANSFERS_URL,
            "min_timestamp",
            &TransferSaver,
        )
        .await;
    }
}
