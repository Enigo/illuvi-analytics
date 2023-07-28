use crate::api_reader::coingecko::coins_reader;
use crate::api_reader::etherscan::transactions_reader;
use crate::api_reader::immutablex::{
    assets_reader, collection_reader, enricher, mints_reader, orders_reader, transfers_reader,
};
use crate::db::db_handler;
use log::info;
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
    let collections = collection_reader::read_collections(pool).await;
    for collection in collections {
        info!("Starting for {collection}");
        mints_reader::read_mints(&collection, pool).await;
        assets_reader::read_assets(&collection, pool).await;
        orders_reader::read_orders(&collection, pool).await;
        transfers_reader::read_transfers(&collection, pool).await;
        info!("Done with {collection}");
    }
    enricher::enrich(pool).await;
}

async fn read_etherscan(pool: &Pool<Postgres>) {
    transactions_reader::read_land_transactions(pool).await;
}

async fn read_coingecko(pool: &Pool<Postgres>) {
    coins_reader::read_coins(pool).await;
}
