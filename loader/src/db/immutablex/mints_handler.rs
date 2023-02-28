use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::mint::Mint;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::{query_as, Pool, Postgres, QueryBuilder};

pub struct MintSaver;

#[async_trait]
impl Persistable<Mint> for MintSaver {
    async fn create_one(&self, mint: &Mint, pool: &Pool<Postgres>) {
        let mint_result = &mint.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into mint (transaction_id, status, wallet, token_type, token_id, minted_on) ",
        );

        query_builder.push_values(mint_result, |mut builder, res| {
            builder
                .push_bind(res.transaction_id)
                .push_bind(res.status.clone())
                .push_bind(res.wallet.clone())
                .push_bind(res.token.the_type.clone())
                .push_bind(res.token.data.token_id.parse::<i32>().unwrap())
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
