use crate::api_reader::{assets_reader, mints_reader};
use crate::env_utils;

#[tokio::main]
pub async fn read() {
    if env_utils::as_parsed::<bool>("MINTS_ENABLED") {
        mints_reader::read().await;
    }
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        assets_reader::read().await;
    }
}
