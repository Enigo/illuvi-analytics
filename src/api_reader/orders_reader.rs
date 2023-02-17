use crate::api_reader::api_utils;
use crate::db::orders_handler::OrderSaver;
use crate::db::{db_handler, orders_handler};
use crate::model::order::{Order, SingleOrder};
use crate::model::trade::Trade;
use crate::utils::env_utils;
use crate::utils::price_utils;
use futures::StreamExt;
use log::{error, info};
use sqlx::{Pool, Postgres};

const ORDERS_URL: &str = "https://api.x.immutable.com/v1/orders?sell_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&order_by=updated_at&direction=asc&updated_min_timestamp=";
const ORDER_URL: &str = "https://api.x.immutable.com/v1/orders";
const TRADES_URL: &str = "https://api.x.immutable.com/v1/trades?party_b_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200&party_b_token_id=";
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

        api_utils::fetch_and_persist_all_api_responses_with_cursor::<Order>(
            url.as_str(),
            &OrderSaver,
        )
        .await;

        enrich_order_data().await;
    }
}

async fn enrich_order_data() {
    let pool = db_handler::open_connection().await;
    update_symbols(&pool).await;
    enrich_wallet_to_and_sell_price(&pool).await;
    db_handler::close_connection(pool).await;
}

// ORDERS_URL doesn't currently return `symbols` in the response, so it should be updated separately
// https://forum.immutable.com/t/order-api-symbol-field/354
async fn update_symbols(pool: &Pool<Postgres>) {
    async fn process_id(order_id: i32, pool: &Pool<Postgres>) {
        let url = format!("{}/{}", ORDER_URL, order_id);
        let response = api_utils::fetch_single_api_response::<SingleOrder>(url.as_str()).await;
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

// There is currently no other way but to go thru multiple API calls
// https://forum.immutable.com/t/how-to-get-wallet-that-bough-given-asset/359/7
async fn enrich_wallet_to_and_sell_price(pool: &Pool<Postgres>) {
    async fn process_id(token_id: i32, pool: &Pool<Postgres>) {
        let url = format!("{}{}", TRADES_URL, token_id);
        let all_trades =
            api_utils::fetch_all_api_responses_with_cursor::<Trade>(url.as_str()).await;
        for trade in all_trades {
            // all_trades return trades that might have been already updated (since it is all trades for the given token_id)
            // need to query only order_ids with no wallet_to/sell_price
            let unprocessed_orders = orders_handler::fetch_all_filled_order_ids_with_no_wallet_to_and_no_sell_price_for_token_id(token_id, &pool)
                .await
                .unwrap_or_default();
            for single_trade in trade.result {
                let seller_order_id = single_trade.seller.order_id;
                if unprocessed_orders.contains(&seller_order_id) {
                    let url = format!("{}/{}", ORDER_URL, single_trade.buyer.order_id);
                    match api_utils::fetch_single_api_response::<SingleOrder>(url.as_str()).await {
                        Ok(order) => {
                            let sell_data = order.sell.data;
                            orders_handler::update_wallet_to_and_sell_price_for_order_id(
                                seller_order_id,
                                order.wallet,
                                price_utils::get_price(
                                    &sell_data.quantity,
                                    sell_data.decimals.unwrap(),
                                ),
                                &pool,
                            )
                            .await;
                        }
                        Err(e) => {
                            error!("Order API {} cannot be parsed! {}", url, e)
                        }
                    };
                }
            }
        }
    }

    match orders_handler::fetch_all_filled_token_ids_with_no_wallet_to_and_no_sell_price(&pool)
        .await
    {
        Some(token_ids) => {
            info!(
                "Updating wallet_to and sell_price for {} tokens",
                token_ids.len()
            );
            let mut futures = futures::stream::iter(token_ids)
                .map(|token_id| process_id(token_id, &pool))
                .buffer_unordered(3);

            while let Some(_) = futures.next().await {}
        }
        None => {
            info!("No wallet_to and sell_price to upgrade!")
        }
    }
}
