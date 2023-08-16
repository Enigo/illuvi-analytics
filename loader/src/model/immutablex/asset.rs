use crate::model::immutablex::shared::PaginatedApi;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub result: Vec<TheResult>,
    pub cursor: String,
}

impl PaginatedApi for Asset {
    fn get_cursor(&self) -> String {
        self.cursor.clone()
    }

    fn has_results(&self) -> bool {
        !self.result.is_empty()
    }
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub token_id: String,
    pub token_address: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "user")]
    pub current_owner: String,
    pub created_at: String,
    pub updated_at: String,
}
