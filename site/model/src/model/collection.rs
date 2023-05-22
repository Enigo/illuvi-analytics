use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionData {
    pub address: String,
    pub name: String,
    pub collection_image_url: String,
    pub description: String,
    pub created_on: NaiveDateTime,
}
