use crate::db::db_handler;
use crate::db::db_handler::Persistable;
use crate::model::shared::PaginatedApi;
use log::{error, info};
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};

pub async fn read_with_cursor_as<T: DeserializeOwned + PaginatedApi>(
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
    let response = fetch_api_response::<T>(url.as_str()).await;
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

pub async fn fetch_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}
