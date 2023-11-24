use crate::api_reader::api_utils::fetch_single_api_response;
use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::shared::PaginatedApi;
use crate::utils::env_utils;
use log::info;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};

const FALLBACK_LAST_TIMESTAMP: &str = "2000-01-12T02:00:00Z";

pub async fn fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp<
    T: DeserializeOwned + PaginatedApi,
>(
    pool: &Pool<Postgres>,
    url: &str,
    last_timestamp_url_param: &str,
    token_address: &String,
    persistable: &dyn Persistable<T>,
) {
    let last_timestamp = match persistable.get_last_timestamp(pool, token_address).await {
        None => String::from(FALLBACK_LAST_TIMESTAMP),
        Some(value) => value.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    };
    let complete_url = format!("{}&{}={}", url, last_timestamp_url_param, last_timestamp);

    let mut cursor = None;
    loop {
        let result = fetch_and_get_result::<T>(complete_url.as_str(), cursor).await;
        if result.is_none() {
            break;
        } else {
            let res = result.unwrap();
            if res.has_results() {
                persistable.create_one(&res, pool).await;
            }
            cursor = Some(res.get_cursor());
        }
    }
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

pub fn get_immutable_x_api_header() -> Vec<(&'static str, String)> {
    let api_key = env_utils::as_string("IMMUTABLE_X_API_KEY");
    vec![("x-api-key", api_key)]
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
    let response =
        fetch_single_api_response::<T>(url.as_str(), &get_immutable_x_api_header()).await;
    match response {
        Some(result) => {
            info!("Processing response for {url}");
            if !result.get_cursor().is_empty() {
                return Some(result);
            }
            None
        }
        _ => None,
    }
}
