use crate::api_reader::immutablex::{assets_reader, orders_reader};
use crate::db::immutablex::mints_handler;
use sqlx::{Pool, Postgres};

pub async fn enrich(pool: &Pool<Postgres>) {
    mints_handler::update_d1sk_price(pool).await;
    assets_reader::update_metadata(pool).await;
    orders_reader::check_orders_consistency(pool).await;
}
