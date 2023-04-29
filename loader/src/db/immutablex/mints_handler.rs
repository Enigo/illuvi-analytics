use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::mint::Mint;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::{query, query_as, query_scalar, Pool, Postgres, QueryBuilder};

pub struct MintSaver;

#[async_trait]
impl Persistable<Mint> for MintSaver {
    async fn create_one(&self, mint: &Mint, pool: &Pool<Postgres>) {
        let mint_result = &mint.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into mint (transaction_id, status, wallet, token_type, token_id, token_address, minted_on) ",
        );

        query_builder.push_values(mint_result, |mut builder, res| {
            let token_data = &res.token.data;
            builder
                .push_bind(res.transaction_id)
                .push_bind(res.status.clone())
                .push_bind(res.wallet.clone())
                .push_bind(res.token.the_type.clone())
                .push_bind(token_data.token_id.parse::<i32>().unwrap())
                .push_bind(token_data.token_address.clone())
                .push_bind(DateTime::parse_from_rfc3339(&res.minted_on).unwrap());
        });

        let query = query_builder
            .push(" ON CONFLICT (transaction_id) DO NOTHING")
            .build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {e}")
            }
        }
    }

    async fn get_last_timestamp(&self, pool: &Pool<Postgres>) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) = query_as("select max(minted_on) from mint")
            .fetch_one(pool)
            .await
            .unwrap_or_else(|e| {
                error!("Couldn't fetch data! {e}");
                (None,)
            });

        result.0
    }
}

pub async fn fetch_all_lands_with_no_price_or_currency(
    pool: &Pool<Postgres>,
) -> Option<Vec<String>> {
    return match query_scalar(
        "select distinct(wallet) from mint where (price is null or currency is null) and token_address='0x9e0d99b864e1ac12565125c5a82b59adea5a09cd'"
    )
        .fetch_all(pool)
        .await {
        Ok(wallets) => {
            Some(wallets)
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

pub async fn update_price_and_currency_for_wallet(
    wallet: &str,
    price: f32,
    currency: String,
    pool: &Pool<Postgres>,
) {
    match query("update mint set price = $1, currency = $2 where wallet = $3")
        .bind(price)
        .bind(currency)
        .bind(wallet)
        .execute(pool)
        .await
    {
        Ok(_) => {
            info!("Updated wallet {wallet}")
        }
        Err(e) => {
            error!("Error updating order_id {wallet}: {e}");
        }
    }
}

pub async fn update_price_and_currency_for_wallet_and_token_id(
    wallet: &str,
    price: f32,
    currency: String,
    token_id: &i32,
    pool: &Pool<Postgres>,
) {
    match query("update mint set price = $1, currency = $2 where wallet = $3 and token_id = $4")
        .bind(price)
        .bind(currency)
        .bind(wallet)
        .bind(token_id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            info!("Updated wallet {wallet} and token_id {token_id}")
        }
        Err(e) => {
            error!("Error updating order_id {wallet} and token_id {token_id}: {e}");
        }
    }
}
