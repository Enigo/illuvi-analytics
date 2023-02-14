use crate::api_reader::utils;
use crate::db::orders_handler::OrderSaver;
use crate::db::{db_handler, orders_handler};
use crate::env_utils;
use crate::model::order::{Order, SingleOrder};
use futures::StreamExt;
use log::{error, info};
use sqlx::{Pool, Postgres};

const ORDERS_URL: &str = "https://api.x.immutable.com/v1/orders?sell_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&order_by=created_at&direction=asc&updated_min_timestamp=";
const ORDER_URL: &str = "https://api.x.immutable.com/v1/orders";
const FALLBACK_UPDATED_ON: &str = "2000-01-12T02:00:00Z";
const ERC20: &str = "ERC20";

pub async fn read_orders() {
    if env_utils::as_parsed::<bool>("ORDERS_ENABLED") {
        let updated_on = match orders_handler::fetch_last_updated_on().await {
            None => String::from(FALLBACK_UPDATED_ON),
            Some(value) => value.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };

        let url = ORDERS_URL.to_string() + updated_on.as_str();
        info!(
            "Reading orders with updated_min_timestamp {} url {}",
            updated_on, url
        );

        utils::read_with_cursor_as::<Order>(url.as_str(), &OrderSaver).await;

        enrich_order_data().await;
    }
}

async fn enrich_order_data() {
    let pool = db_handler::open_connection().await;
    update_symbols(&pool).await;
    db_handler::close_connection(pool).await;
}

// ORDERS_URL doesn't currently return `symbols` in the response, so it should be updated separately
// https://forum.immutable.com/t/order-api-symbol-field/354
async fn update_symbols(pool: &Pool<Postgres>) {
    async fn process_id(order_id: i32, pool: &Pool<Postgres>) {
        let url = format!("{}/{}", ORDER_URL, order_id);
        let response = utils::fetch_api_response::<SingleOrder>(url.as_str()).await;
        match response {
            Ok(order) => {
                orders_handler::update_buy_currency_for_order_id(
                    order_id,
                    &order.buy.data.symbol.unwrap(),
                    &pool,
                )
                    .await;
            }
            Err(e) => {
                error!("Order API {} cannot be parsed! {}", url, e)
            }
        }
    }

    match orders_handler::fetch_all_order_ids_for_buy_currency(ERC20, &pool).await {
        Some(order_ids) => {
            info!("Updating symbols for {} orders", order_ids.len());
            // API limit is 5RPS
            let mut futures = futures::stream::iter(order_ids)
                .map(|order_id| process_id(order_id, &pool))
                .buffer_unordered(3);

            // waiting for all to complete
            while let Some(_) = futures.next().await {}
        }
        None => {
            info!("No token ids for {} found!", ERC20)
        }
    }
}
