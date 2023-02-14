use crate::api_reader::utils;
use crate::db::mints_handler::MintSaver;
use crate::env_utils;

const MINTS_URL: &str = "https://api.x.immutable.com/v1/mints?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

pub async fn read_mints() {
    if env_utils::as_parsed::<bool>("MINTS_ENABLED") {
        utils::read_with_cursor_as(MINTS_URL, &MintSaver).await
    }
}
