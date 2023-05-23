use crate::model::immutablex::shared::PaginatedApi;
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
    pub maker_fees: MakerFees,
    pub taker_fees: TakerFees,
    pub timestamp: String,
    pub updated_timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct Sell {
    pub data: SellData,
}

#[derive(Deserialize, Debug)]
pub struct SellData {
    pub token_id: Option<String>,
    pub token_address: String,
    pub decimals: Option<i32>,
    pub quantity: String,
}

#[derive(Deserialize, Debug)]
pub struct MakerFees {
    pub quantity_with_fees: String,
    pub decimals: i32,
}

#[derive(Deserialize, Debug)]
pub struct TakerFees {
    pub symbol: String,
    pub quantity_with_fees: String,
    pub decimals: i32,
}

#[derive(Deserialize, Debug)]
pub struct SingleOrder {
    pub order_id: i32,
    #[serde(rename = "user")]
    pub wallet: String,
}
