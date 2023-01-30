use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub token_id: String,
    pub token_address: String,
    pub metadata: Metadata,
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
    pub image_url: String,
}
