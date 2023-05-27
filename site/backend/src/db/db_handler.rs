use crate::utils::env_utils;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres};

pub async fn create_pool() -> Pool<Postgres> {
    let options = PgConnectOptions::new()
        .host(env_utils::as_string("DB_HOST").as_str())
        .port(env_utils::as_parsed::<u16>("DB_PORT"))
        .database(env_utils::as_string("DB_DATABASE").as_str())
        .username(env_utils::as_string("DB_USERNAME").as_str())
        .password(env_utils::as_string("DB_PASSWORD").as_str())
        .statement_cache_capacity(1000)
        .disable_statement_logging()
        .clone();

    PgPoolOptions::new()
        .min_connections(10)
        .max_connections(190)
        .connect_with(options)
        .await
        .expect("DB is not accessible!")
}
