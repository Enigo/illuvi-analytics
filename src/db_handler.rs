use std::env;
use log::{error, info};
use sqlx::{ConnectOptions, Pool, Postgres, QueryBuilder};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::types::chrono::DateTime;

use crate::model::mint::TheResult;

pub async fn open_connection() -> Pool<Postgres> {
    let options = PgConnectOptions::new()
        .host(env::var("DB_HOST").expect("DB_HOST should be set").as_str())
        .port(env::var("DB_PORT").expect("DB_PORT should be set").parse().expect("DB_PORT should be a valid u16 value"))
        .database(env::var("DB_DATABASE").expect("DB_DATABASE should be set").as_str())
        .username(env::var("DB_USERNAME").expect("DB_USERNAME should be set").as_str())
        .password(env::var("DB_PASSWORD").expect("DB_PASSWORD should be set").as_str())
        .disable_statement_logging()
        .clone();

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("DB is not accessible!")
}

pub async fn close_connection(pool: Pool<Postgres>) {
    pool.close().await;
}

pub async fn save_mints(mint_result: Vec<TheResult>, connection: &Pool<Postgres>) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into mint (transaction_id, status, wallet, token_type, token_id, minted_on) "
    );

    query_builder.push_values(mint_result, |mut builder, res| {
        builder.push_bind(res.transaction_id)
            .push_bind(res.status.clone())
            .push_bind(res.wallet.clone())
            .push_bind(res.token.the_type.clone())
            .push_bind(res.token.data.token_id.clone())
            .push_bind(DateTime::parse_from_rfc3339(&res.minted_on).unwrap());
    });

    let query = query_builder.build();
    match query.execute(connection)
        .await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {}", e)
        }
    }
}