use crate::model::shared::PaginatedApi;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Order {
    pub result: Vec<TheResult>,
    pub cursor: String,
}

impl PaginatedApi for Order {
    fn get_cursor(&self) -> String {
        self.cursor.clone()
    }

    fn has_results(&self) -> bool {
        !self.result.is_empty()
    }
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub order_id: i32,
    pub status: String,
    #[serde(rename = "user")]
    pub wallet: String,
    pub sell: Sell,
    pub buy: Buy,
    pub timestamp: String,
    pub updated_timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct Sell {
    pub data: SellData,
}

#[derive(Deserialize, Debug)]
pub struct Buy {
    pub data: BuyData,
    #[serde(rename = "type")]
    pub buy_currency: String,
}

#[derive(Deserialize, Debug)]
pub struct BuyData {
    pub decimals: i32,
    pub quantity: String,
    pub symbol: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SellData {
    pub token_id: String,
    pub token_address: String,
}

#[derive(Deserialize, Debug)]
pub struct SingleOrder {
    pub order_id: i32,
    pub buy: Buy,
}
