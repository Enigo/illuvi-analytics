use crate::model::mint::TheResult;
use log::{error, info};
use sqlx::types::chrono::DateTime;
use sqlx::{query_as, FromRow, Pool, Postgres, QueryBuilder};
use std::collections::HashSet;

pub async fn save_mints(mint_result: Vec<TheResult>, connection: &Pool<Postgres>) {
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
    match query.execute(connection).await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {}", e)
        }
    }
}

pub async fn get_all_token_ids(connection: &Pool<Postgres>) -> Option<HashSet<i32>> {
    match query_as::<Postgres, MintDb>("select token_id from mint")
        .fetch_all(connection)
        .await
    {
        Ok(result) => Some(result.iter().map(|mint| mint.token_id).collect()),
        Err(e) => {
            error!("Error fetching data {}", e);
            None
        }
    }
}

#[derive(FromRow)]
struct MintDb {
    token_id: i32,
}
