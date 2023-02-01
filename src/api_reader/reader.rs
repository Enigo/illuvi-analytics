use crate::api_reader::utils;
use crate::db::assets_handler::AssetSaver;
use crate::db::db_handler;
use crate::db::db_handler::Persistable;
use crate::db::mints_handler::MintSaver;
use crate::env_utils;
use crate::model::asset::Asset;
use crate::model::mint::Mint;
use crate::model::shared::PaginatedApi;
use log::{error, info};
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};

const MINTS_URL: &str = "https://api.x.immutable.com/v1/mints?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";
const ASSETS_URL: &str =
    "https://api.x.immutable.com/v1/assets?collection=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

#[tokio::main]
pub async fn read() {
    if env_utils::as_parsed::<bool>("MINTS_ENABLED") {
        read_with_cursor_as::<Mint>(MINTS_URL, &MintSaver).await;
    }
    if env_utils::as_parsed::<bool>("ASSETS_ENABLED") {
        read_with_cursor_as::<Asset>(ASSETS_URL, &AssetSaver).await;
    }
}

async fn read_with_cursor_as<T: DeserializeOwned + PaginatedApi>(
    url: &str,
    persistable: &dyn Persistable<T>,
) {
    let pool = db_handler::open_connection().await;
    let mut cursor = None;
    loop {
        cursor = execute_and_get_cursor::<T>(url, cursor, persistable, &pool).await;
        if cursor.is_none() {
            break;
        } else {
            info!("Current cursor: {}", cursor.clone().unwrap());
        }
    }
    db_handler::close_connection(pool).await;
}

async fn execute_and_get_cursor<T: DeserializeOwned + PaginatedApi>(
    url: &str,
    cursor: Option<String>,
    persistable: &dyn Persistable<T>,
    pool: &Pool<Postgres>,
) -> Option<String> {
    let url = if cursor.is_some() {
        url.to_owned() + "&cursor=" + cursor.unwrap().as_str()
    } else {
        String::from(url)
    };
    let response = utils::fetch_api_response::<T>(url.as_str()).await;
    match response {
        Ok(result) => {
            info!("Processing response");
            if result.has_results() {
                persistable.persist_one(&result, pool).await;
            }

            if !result.get_cursor().is_empty() {
                return Some(result.get_cursor());
            }
            None
        }
        Err(e) => {
            error!("{} API response cannot be parsed! {}", url, e);
            None
        }
    }
}
