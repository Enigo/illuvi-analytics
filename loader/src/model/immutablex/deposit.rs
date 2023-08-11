use crate::model::immutablex::shared::PaginatedApi;
use crate::model::immutablex::token::Token;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Deposit {
    pub result: Vec<TheResult>,
    pub cursor: String,
}

impl PaginatedApi for Deposit {
    fn get_cursor(&self) -> String {
        self.cursor.clone()
    }

    fn has_results(&self) -> bool {
        !self.result.is_empty()
    }
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub transaction_id: i32,
    pub status: String,
    #[serde(rename = "user")]
    pub wallet: String,
    pub timestamp: String,
    pub token: Token,
}
