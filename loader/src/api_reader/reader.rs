use crate::api_reader::coingecko::coins_reader;
use crate::api_reader::etherscan::transactions_reader;
use crate::api_reader::immutablex::{
    assets_reader, collection_reader, mints_reader, orders_reader, transfers_reader,
};
use crate::db::db_handler;
use sqlx::{Pool, Postgres};

#[tokio::main]
pub async fn read() {
    let pool = db_handler::open_connection().await;

    read_immutablex(&pool).await;
    read_etherscan(&pool).await;
    read_coingecko(&pool).await;

    db_handler::refresh_mat_views(&pool).await;
    db_handler::close_connection(pool).await;
}

async fn read_immutablex(pool: &Pool<Postgres>) {
    collection_reader::read_collections(pool).await;
    mints_reader::read_mints(pool).await;
    assets_reader::read_assets(pool).await;
    orders_reader::read_orders(pool).await;
    transfers_reader::read_transfers(pool).await;
}

async fn read_etherscan(pool: &Pool<Postgres>) {
    transactions_reader::read_transactions(pool).await;
}

async fn read_coingecko(pool: &Pool<Postgres>) {
    coins_reader::read_coins(pool).await;
}
