use crate::api_reader::api_utils;
use crate::api_reader::immutablex::utils;
use crate::db::immutablex::orders_handler;
use crate::db::immutablex::orders_handler::OrderDb;
use crate::db::immutablex::orders_handler::OrderSaver;
use crate::model::immutablex::order::{Order, SingleOrder};
use crate::model::immutablex::trade::Trade;
use crate::utils::{env_utils, price_utils};
use futures::StreamExt;
use log::info;
use sqlx::{Pool, Postgres};

const ORDERS_URL: &str = "https://api.x.immutable.com/v3/orders?page_size=200&order_by=updated_at&direction=asc&sell_token_address=";
const ORDER_URL: &str = "https://api.x.immutable.com/v3/orders";

pub async fn read_orders(token_address: &String, pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("ORDERS_ENABLED") {
        utils::fetch_and_persist_all_api_responses_with_cursor_and_last_timestamp::<Order>(
            pool,
            format!("{}{}", ORDERS_URL, token_address).as_str(),
            "updated_min_timestamp",
            token_address,
            &OrderSaver,
        )
        .await;

        enrich_wallet_to_and_transaction_id(token_address, &pool).await;
    }
}

// this is a helper method to check orders consistency
// last time - 29-10-2023 - the entire order_data was checked
// it was done with a help of an additional boolean column 'checked' and it took 5 days
// if another consistency check is required most likely the data before the last check should be skipped
pub async fn check_orders_consistency(pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("ORDERS_CONSISTENCY_ENABLED") {
        async fn process_id(order_db: OrderDb, pool: &Pool<Postgres>) {
            let order_id = order_db.order_id;
            let url = format!("{}/{}", ORDER_URL, order_id);
            match api_utils::fetch_single_api_response::<SingleOrder>(
                url.as_str(),
                &utils::get_immutable_x_api_header(),
            )
            .await
            {
                Some(order) => {
                    let maker_fees = order.maker_fees;
                    let taker_fees = order.taker_fees;
                    let sell_price =
                        price_utils::get_price(&maker_fees.quantity_with_fees, maker_fees.decimals);
                    let buy_price =
                        price_utils::get_price(&taker_fees.quantity_with_fees, taker_fees.decimals);

                    let mut to_update = false;
                    let sell_price_db = order_db.sell_price;
                    if sell_price_db != sell_price {
                        info!("For order {order_id} not equal sell prices! db {sell_price_db} vs api {sell_price}");
                        to_update = true;
                    }
                    let buy_price_db = order_db.buy_price;
                    if buy_price_db != buy_price {
                        info!("For order {order_id} not equal buy prices! db {buy_price_db} vs api {buy_price}");
                        to_update = true;
                    }
                    if to_update {
                        orders_handler::update_buy_price_and_sell_price_for_order_id(
                            order_id, buy_price, sell_price, pool,
                        )
                        .await;
                    }
                }
                None => {}
            };
        }

        let orders = orders_handler::fetch_all_not_checked_order_ids(pool).await;
        info!("Checking orders consistency for {} orders", orders.len());
        let mut futures = futures::stream::iter(orders)
            .map(|order| process_id(order, &pool))
            .buffer_unordered(45);

        while let Some(_) = futures.next().await {}
    }
}

// There is currently no other way but to go thru multiple API calls
// https://forum.immutable.com/t/how-to-get-wallet-that-bough-given-asset/359/7
async fn enrich_wallet_to_and_transaction_id(token_address: &String, pool: &Pool<Postgres>) {
    async fn process_id(token_address: &String, token_id: i32, pool: &Pool<Postgres>) {
        let url = format!("https://api.x.immutable.com/v3/trades?&page_size=200&party_b_token_id={}&party_b_token_address={}", token_id, token_address);
        let all_trades = utils::fetch_all_api_responses_with_cursor::<Trade>(url.as_str()).await;
        for trade in all_trades {
            // all_trades return trades that might have been already updated (since it is all trades for the given token_id)
            // need to query only order_ids with no wallet_to/transaction_id
            let unprocessed_orders = orders_handler::fetch_all_filled_order_ids_with_no_wallet_to_or_transaction_id_for_token_id(token_id, &pool)
                .await
                .unwrap_or_default();
            for single_trade in trade.result {
                let seller_order_id = single_trade.seller.order_id;
                if unprocessed_orders.contains(&seller_order_id) {
                    let url = format!("{}/{}", ORDER_URL, single_trade.buyer.order_id);
                    match api_utils::fetch_single_api_response::<SingleOrder>(
                        url.as_str(),
                        &utils::get_immutable_x_api_header(),
                    )
                    .await
                    {
                        Some(order) => {
                            orders_handler::update_wallet_to_and_transaction_id_for_order_id(
                                seller_order_id,
                                order.wallet,
                                single_trade.transaction_id,
                                &pool,
                            )
                            .await;
                        }
                        None => {}
                    };
                }
            }
        }
    }

    match orders_handler::fetch_all_filled_token_ids_with_no_wallet_to_or_transaction_id(
        token_address,
        &pool,
    )
    .await
    {
        Some(token_ids) => {
            info!(
                "Updating wallet_to and transaction_id for {} tokens",
                token_ids.len()
            );
            let mut futures = futures::stream::iter(token_ids)
                .map(|token_id| process_id(token_address, token_id, &pool))
                .buffer_unordered(45);

            while let Some(_) = futures.next().await {}
        }
        None => {
            info!("No wallet_to or sell_price or transaction_id to upgrade!")
        }
    }
}
