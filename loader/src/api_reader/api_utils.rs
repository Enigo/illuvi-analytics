use log::error;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub async fn fetch_single_api_response<T: DeserializeOwned>(endpoint: &str) -> Option<T> {
    let response = reqwest::get(endpoint).await;

    match response {
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
