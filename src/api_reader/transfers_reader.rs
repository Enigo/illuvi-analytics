use crate::api_reader::api_utils;
use crate::db::transfers_handler;
use crate::db::transfers_handler::TransferSaver;
use crate::model::transfer::Transfer;
use crate::utils::env_utils;
use log::info;

const TRANSFERS_URL: &str = "https://api.x.immutable.com/v1/transfers?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&order_by=created_at&direction=asc&min_timestamp=";
const FALLBACK_CREATED_ON: &str = "2000-01-12T02:00:00Z";

pub async fn read_transfers() {
    if env_utils::as_parsed::<bool>("TRANSFERS_ENABLED") {
        let created_on = match transfers_handler::fetch_last_created_on().await {
            None => String::from(FALLBACK_CREATED_ON),
            Some(value) => value.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };

        let url = TRANSFERS_URL.to_string() + created_on.as_str();
        info!(
            "Reading transfers with min_timestamp {} url {}",
            created_on, url
        );

        api_utils::fetch_and_persist_all_api_responses_with_cursor::<Transfer>(
            url.as_str(),
            &TransferSaver,
        )
        .await;
    }
}
