use crate::db::db_handler::Persistable;
use crate::model::mint::Mint;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::DateTime;
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct MintSaver;

#[async_trait]
impl Persistable<Mint> for MintSaver {
    async fn persist_one(&self, mint: &Mint, pool: &Pool<Postgres>) {
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

        let query = query_builder.build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {}", e)
            }
        }
    }
}
