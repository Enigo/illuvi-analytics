use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::transfer::Transfer;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::{query_as, Pool, Postgres, QueryBuilder};

pub struct TransferSaver;

#[async_trait]
impl Persistable<Transfer> for TransferSaver {
    async fn create_one(&self, transfer: &Transfer, pool: &Pool<Postgres>) {
        let transfer_result = &transfer.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into transfer (transaction_id, status, wallet_from, wallet_to, token_id, token_address, created_on) ",
        );
        query_builder.push_values(transfer_result, |mut builder, res| {
            let token_data = &res.token.data;
            builder
                .push_bind(res.transaction_id)
                .push_bind(res.status.clone())
                .push_bind(res.wallet_from.clone())
                .push_bind(res.wallet_to.clone())
                .push_bind(token_data.token_id.clone().parse::<i32>().unwrap())
                .push_bind(&token_data.token_address)
                .push_bind(DateTime::parse_from_rfc3339(&res.timestamp).unwrap());
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

    async fn get_last_timestamp(
        &self,
        pool: &Pool<Postgres>,
        token_address: &String,
    ) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) =
            query_as("select max(created_on) from transfer where token_address=$1")
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
