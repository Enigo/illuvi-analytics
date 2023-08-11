use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Token {
    pub data: TokenData,
}

#[derive(Deserialize, Debug)]
pub struct TokenData {
    pub token_id: String,
    pub token_address: String,
}
