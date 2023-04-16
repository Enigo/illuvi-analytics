use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionData {
    pub address: String,
    pub name: String,
    pub collection_image_url: String,
}
