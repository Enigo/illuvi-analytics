use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::order::Order;
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
            "insert into order_data (order_id, status, wallet_from, token_id, token_address, buy_currency, sell_price, buy_price, created_on, updated_on) ",
        );

        query_builder.push_values(order_result, |mut builder, res| {
            let sell_data = &res.sell.data;
            let maker_fees = &res.maker_fees;
            let taker_fees = &res.taker_fees;
            let token_id = sell_data.token_id.clone().unwrap().parse::<i32>().unwrap();
            builder
                .push_bind(res.order_id)
                .push_bind(res.status.clone())
                .push_bind(res.wallet.clone())
                .push_bind(token_id)
                .push_bind(&sell_data.token_address)
                .push_bind(&taker_fees.symbol)
                .push_bind(price_utils::get_price(
                    &maker_fees.quantity_with_fees,
                    maker_fees.decimals,
                ))
                .push_bind(price_utils::get_price(
                    &taker_fees.quantity_with_fees,
                    taker_fees.decimals,
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
                error!("Couldn't insert values due to {e}")
            }
        }
    }

    async fn get_last_timestamp(
        &self,
        pool: &Pool<Postgres>,
        token_address: &String,
    ) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) =
            query_as("select max(updated_on) from order_data where token_address=$1")
                .bind(token_address)
                .fetch_one(pool)
                .await
                .unwrap_or_else(|e| {
                    error!("Couldn't fetch data! {e}");
                    (None,)
                });

        result.0
    }
}

pub async fn fetch_all_filled_token_ids_with_no_wallet_to_or_transaction_id(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> Option<Vec<i32>> {
    return match query_scalar(
        "select distinct(token_id) from order_data where status = 'filled' and (wallet_to is null or transaction_id is null) and token_address=$1"
    )
        .bind(token_address)
        .fetch_all(pool)
        .await {
        Ok(token_ids) => {
            Some(token_ids)
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

pub async fn fetch_all_filled_order_ids_with_no_wallet_to_or_transaction_id_for_token_id(
    token_id: i32,
    pool: &Pool<Postgres>,
) -> Option<Vec<i32>> {
    return match query_scalar(
        "select order_id from order_data where status = 'filled' and (wallet_to is null or transaction_id is null) and token_id = $1"
    )
        .bind(token_id)
        .fetch_all(pool)
        .await {
        Ok(token_ids) => {
            Some(token_ids)
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

pub async fn update_wallet_to_and_transaction_id_for_order_id(
    order_id: i32,
    wallet_to: String,
    transaction_id: i32,
    pool: &Pool<Postgres>,
) {
    match query("update order_data set wallet_to = $1, transaction_id = $2 where order_id = $3")
        .bind(wallet_to)
        .bind(transaction_id)
        .bind(order_id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            info!("Updated order_id {order_id}")
        }
        Err(e) => {
            error!("Error updating order_id {order_id}: {e}");
        }
    }
}
