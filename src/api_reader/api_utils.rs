use crate::db::db_handler;
use crate::db::db_handler::Persistable;
use crate::model::shared::PaginatedApi;
use log::{error, info};
use serde::de::DeserializeOwned;

pub async fn fetch_and_persist_all_api_responses_with_cursor<T: DeserializeOwned + PaginatedApi>(
    url: &str,
    persistable: &dyn Persistable<T>,
) {
    let pool = db_handler::open_connection().await;
    let mut cursor = None;
    loop {
        let result = fetch_and_get_result::<T>(url, cursor).await;
        if result.is_none() {
            break;
        } else {
            let res = result.unwrap();
            if res.has_results() {
                persistable.persist_one(&res, &pool).await;
            }
            cursor = Some(res.get_cursor());
        }
    }
    db_handler::close_connection(pool).await;
}

pub async fn fetch_all_api_responses_with_cursor<T: DeserializeOwned + PaginatedApi>(
    url: &str,
) -> Vec<T> {
    let mut cursor = None;
    let mut results = Vec::new();
    loop {
        let result = fetch_and_get_result::<T>(url, cursor).await;
        if result.is_none() {
            break;
        } else {
            let res = result.unwrap();
            cursor = Some(res.get_cursor());
            results.push(res);
        }
    }

    results
}

async fn fetch_and_get_result<T: DeserializeOwned + PaginatedApi>(
    url: &str,
    cursor: Option<String>,
) -> Option<T> {
    let url = if cursor.is_some() {
        url.to_owned() + "&cursor=" + cursor.unwrap().as_str()
    } else {
        String::from(url)
    };
    let response = fetch_single_api_response::<T>(url.as_str()).await;
    match response {
        Ok(result) => {
            info!("Processing response for {}", url);
            if !result.get_cursor().is_empty() {
                return Some(result);
            }
            None
        }
        Err(e) => {
            error!("{} API response cannot be parsed! {}", url, e);
            None
        }
    }
}

pub async fn fetch_single_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}
