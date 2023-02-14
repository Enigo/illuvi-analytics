use crate::api_reader::{assets_reader, mints_reader, orders_reader};

#[tokio::main]
pub async fn read() {
    mints_reader::read_mints().await;
    assets_reader::read_assets().await;
    orders_reader::read_orders().await;
}
