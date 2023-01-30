use crate::api_reader::utils;
use futures::StreamExt;
use log::error;
use sqlx::{Pool, Postgres};

use crate::db::{assets_handler, db_handler, mints_handler};
use crate::model::asset::Asset;

const ASSETS_URL: &str =
    "https://api.x.immutable.com/v1/assets/0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";

pub async fn read() {
    let pool = db_handler::open_connection().await;
    match mints_handler::get_all_token_ids(&pool).await {
        Some(token_ids_from_mint) => {
            let token_ids_from_asset = assets_handler::get_all_token_ids(&pool)
                .await
                .unwrap_or_default();
            let ids = &token_ids_from_mint - &token_ids_from_asset;

            // API limit is 5RPS
            let mut futures = futures::stream::iter(ids)
                .map(|id| process_id(id, pool.clone()))
                .buffer_unordered(5);

            // waiting for all to complete
            while let Some(_) = futures.next().await {}
        }
        None => {
            error!("No token ids found in mint!")
        }
    }
    db_handler::close_connection(pool).await;
}

// should prob return a result so that execution if futures can be stopped sooner
async fn process_id(id: i32, pool: Pool<Postgres>) {
    let url = format!("{}/{}", ASSETS_URL, id);
    let response = utils::fetch_api_response::<Asset>(url.as_str()).await;
    match response {
        Ok(asset) => {
            assets_handler::save_asset(&asset, &pool).await;
        }
        Err(e) => {
            error!("Assets API cannot be parsed! {}", e)
        }
    }
}
