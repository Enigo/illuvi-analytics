use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Mint {
    pub result: Vec<TheResult>,
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    #[serde(rename = "timestamp")]
    pub minted_on: String,
    pub transaction_id: i32,
    pub status: String,
    #[serde(rename = "user")]
    pub wallet: String,
    pub token: Token,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    #[serde(rename = "type")]
    pub the_type: String,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub token_id: String,
}