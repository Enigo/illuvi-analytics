use crate::utils::env_utils;
use log::{error, info};
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
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("DB is not accessible!")
}

pub async fn close_connection(pool: Pool<Postgres>) {
    pool.close().await;
}

pub async fn refresh_mat_views(pool: &Pool<Postgres>) {
    let mat_views = vec![
        "asset_current_owner_mat_view",
        "trade_volume_mat_view",
        "trade_volume_full_mat_view",
        "cheapest_and_most_expensive_trades_by_attribute_mat_view",
        "floor_data_mat_by_attribute_view",
        "total_minted_and_burnt_by_attribute_mat_view",
    ];
    for view in mat_views {
        match sqlx::query(&format!("refresh materialized view {}", view))
            .execute(pool)
            .await
        {
            Ok(_) => {
                info!("Successfully refreshed {view}")
            }
            Err(e) => {
                error!("Error {e} refreshing {view}")
            }
        };
    }
}
