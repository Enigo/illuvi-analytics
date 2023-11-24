use log::error;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

pub async fn fetch_single_api_response<T: DeserializeOwned>(
    endpoint: &str,
    headers: &Vec<(&'static str, String)>,
) -> Option<T> {
    let client = Client::new();
    let mut request_builder = client.get(endpoint);

    if !headers.is_empty() {
        let mut request_headers = HeaderMap::new();
        for (name, value) in headers {
            request_headers.insert(*name, HeaderValue::from_str(value.as_str()).unwrap());
        }
        request_builder = request_builder.headers(request_headers);
    }

    match request_builder.send().await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let result = response.json::<T>().await;
                match result {
                    Ok(res) => {
                        return Some(res);
                    }
                    Err(e) => {
                        error!("Error {e} parsing json body for {endpoint}")
                    }
                }
            } else {
                let status_code = response.status();
                let body = response.text().await.unwrap_or(String::from("No body"));
                error!(
                    "Request to {endpoint} failed with status code {status_code} and text '{body}'"
                );
            }
        }
        Err(e) => {
            error!("Error {e} requesting {endpoint}")
        }
    }

    None
}
