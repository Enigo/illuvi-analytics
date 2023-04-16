use crate::api_reader::coingecko::coins_reader;
use crate::api_reader::immutablex::{
    assets_reader, collection_reader, mints_reader, orders_reader, transfers_reader,
};

#[tokio::main]
pub async fn read() {
    read_immutablex().await;
    read_coingecko().await;
}

async fn read_immutablex() {
    collection_reader::read_collections().await;
    mints_reader::read_mints().await;
    assets_reader::read_assets().await;
    orders_reader::read_orders().await;
    transfers_reader::read_transfers().await;
}

async fn read_coingecko() {
    coins_reader::read_coins().await;
}
