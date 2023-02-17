use crate::utils::env_utils;
use async_trait::async_trait;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres};

pub async fn open_connection() -> Pool<Postgres> {
    let options = PgConnectOptions::new()
        .host(env_utils::as_string("DB_HOST").as_str())
        .port(env_utils::as_parsed::<u16>("DB_PORT"))
        .database(env_utils::as_string("DB_DATABASE").as_str())
        .username(env_utils::as_string("DB_USERNAME").as_str())
        .password(env_utils::as_string("DB_PASSWORD").as_str())
        .disable_statement_logging()
        .clone();

    PgPoolOptions::new()
        .max_connections(15)
        .connect_with(options)
        .await
        .expect("DB is not accessible!")
}

pub async fn close_connection(pool: Pool<Postgres>) {
    pool.close().await;
}

#[async_trait]
pub trait Persistable<T> {
    async fn persist_one(&self, result: &T, pool: &Pool<Postgres>);
}
