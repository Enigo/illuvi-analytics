use crate::db::immutablex::mints_handler;
use sqlx::{Pool, Postgres};

pub async fn enrich(pool: &Pool<Postgres>) {
    mints_handler::update_d1sk_price(pool).await
}
