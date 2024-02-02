use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub address: String,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub collection_image_url: String,
    pub project_id: i32,
    pub project_owner_address: String,
    pub metadata_api_url: String,
    pub created_at: String,
    pub updated_at: String,
}
