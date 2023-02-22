use crate::db::db_handler::Persistable;
use crate::model::order::Order;
use crate::utils::price_utils;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::{query, query_as, query_scalar, Pool, Postgres, QueryBuilder};

pub struct OrderSaver;

#[async_trait]
impl Persistable<Order> for OrderSaver {
    async fn create_one(&self, order: &Order, pool: &Pool<Postgres>) {
        let order_result = &order.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into order_data (order_id, status, wallet_from, token_id, token_address, buy_currency, buy_price, created_on, updated_on) ",
        );

        query_builder.push_values(order_result, |mut builder, res| {
            let sell_data = &res.sell.data;
            let buy_data = &res.buy.data;
            let token_id = sell_data.token_id.clone().unwrap().parse::<i32>().unwrap();
            builder
                .push_bind(res.order_id)
                .push_bind(res.status.clone())
                .push_bind(res.wallet.clone())
                .push_bind(token_id)
                .push_bind(&sell_data.token_address)
                .push_bind(&res.buy.buy_currency)
                .push_bind(price_utils::get_price(
                    &buy_data.quantity,
                    buy_data.decimals.unwrap(),
                ))
                .push_bind(DateTime::parse_from_rfc3339(&res.timestamp).unwrap())
                .push_bind(DateTime::parse_from_rfc3339(&res.updated_timestamp).unwrap());
        });

        let query = query_builder.push(
            " ON CONFLICT (order_id) DO UPDATE SET status = EXCLUDED.status, updated_on = EXCLUDED.updated_on;").build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {}", e)
            }
        }
    }

    async fn get_last_timestamp(&self, pool: &Pool<Postgres>) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) = query_as("select max(updated_on) from order_data")
            .fetch_one(pool)
            .await
            .unwrap_or_else(|e| {
                error!("Couldn't fetch data! {}", e);
                (None,)
            });

        result.0
    }
}

pub async fn fetch_all_order_ids_for_buy_currency(
    buy_currency: &str,
    pool: &Pool<Postgres>,
) -> Option<Vec<i32>> {
    return match query_scalar("select order_id from order_data where buy_currency = $1")
        .bind(buy_currency)
        .fetch_all(pool)
        .await
    {
        Ok(order_ids) => Some(order_ids),
        Err(e) => {
            error!("Couldn't fetch ids for currency {}! {}", buy_currency, e);
            None
        }
    };
}

pub async fn update_buy_currency_for_order_id(
    order_id: i32,
    buy_currency: &str,
    pool: &Pool<Postgres>,
) {
    match query("update order_data set buy_currency = $1 where order_id = $2")
        .bind(buy_currency)
        .bind(order_id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            info!("Updated order_id {}", order_id)
        }
        Err(e) => {
            error!("Error updating order_id {}: {}", order_id, e);
        }
    }
}

pub async fn fetch_all_filled_token_ids_with_no_wallet_to_and_no_sell_price(
    pool: &Pool<Postgres>,
) -> Option<Vec<i32>> {
    return match query_scalar(
        "select distinct(token_id) from order_data where status = 'filled' and wallet_to is null and sell_price is null"
    )
        .fetch_all(pool)
        .await {
        Ok(token_ids) => {
            Some(token_ids)
        }
        Err(e) => {
            error!("Error fetching data: {}", e);
            None
        }
    };
}

pub async fn fetch_all_filled_order_ids_with_no_wallet_to_and_no_sell_price_for_token_id(
    token_id: i32,
    pool: &Pool<Postgres>,
) -> Option<Vec<i32>> {
    return match query_scalar(
        "select order_id from order_data where status = 'filled' and wallet_to is null and sell_price is null and token_id = $1"
    )
        .bind(token_id)
        .fetch_all(pool)
        .await {
        Ok(token_ids) => {
            Some(token_ids)
        }
        Err(e) => {
            error!("Error fetching data: {}", e);
            None
        }
    };
}

pub async fn update_wallet_to_and_sell_price_for_order_id(
    order_id: i32,
    wallet_to: String,
    sell_price: f32,
    pool: &Pool<Postgres>,
) {
    match query("update order_data set wallet_to = $1, sell_price = $2 where order_id = $3")
        .bind(wallet_to)
        .bind(sell_price)
        .bind(order_id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            info!("Updated order_id {}", order_id)
        }
        Err(e) => {
            error!("Error updating order_id {}: {}", order_id, e);
        }
    }
}
