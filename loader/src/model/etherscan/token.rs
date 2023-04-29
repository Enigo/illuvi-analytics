use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Token {
    pub status: String,
    pub result: Option<Vec<TheResult>>,
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub hash: String,
    pub value: String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "tokenDecimal")]
    pub token_decimal: String,
}
