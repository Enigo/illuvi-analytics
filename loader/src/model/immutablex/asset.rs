use crate::model::immutablex::shared::PaginatedApi;
use serde::Deserialize;

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
    pub metadata: Metadata,
    #[serde(rename = "user")]
    pub current_owner: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub tier: i32,
    pub solon: i32,
    pub carbon: i32,
    pub crypton: i32,
    pub silicon: i32,
    pub hydrogen: i32,
    pub hyperion: i32,
    pub landmark: String,
}
