use crate::api_reader::immutablex::utils;
use crate::db::immutablex::mints_handler::MintSaver;
use crate::model::immutablex::mint::Mint;
use crate::utils::env_utils;

const MINTS_URL: &str = "https://api.x.immutable.com/v1/mints?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

pub async fn read_mints() {
    if env_utils::as_parsed::<bool>("MINTS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Mint>(
            MINTS_URL,
            "min_timestamp",
            &MintSaver,
        )
        .await
    }
}
