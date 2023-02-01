use crate::model::shared::PaginatedApi;
use serde::de::DeserializeOwned;

pub async fn fetch_api_response<T: DeserializeOwned + PaginatedApi>(
    endpoint: &str,
) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}
