use crate::model::shared::PaginatedApi;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub result: Vec<TheResult>,
    pub cursor: String,
}

impl PaginatedApi for Trade {
    fn get_cursor(&self) -> String {
        self.cursor.clone()
    }

    fn has_results(&self) -> bool {
        !self.result.is_empty()
    }
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    #[serde(rename = "a")]
    pub buyer: Buyer,
    #[serde(rename = "b")]
    pub seller: Seller,
}

#[derive(Deserialize, Debug)]
pub struct Buyer {
    pub order_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Seller {
    pub order_id: i32,
}
