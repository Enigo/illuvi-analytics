use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}
