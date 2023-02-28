use async_trait::async_trait;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait Persistable<T> {
    async fn create_one(&self, result: &T, pool: &Pool<Postgres>);

    async fn get_last_timestamp(&self, pool: &Pool<Postgres>) -> Option<NaiveDateTime>;
}
