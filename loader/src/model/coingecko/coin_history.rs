use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoinHistory {
    pub id: String,
    pub symbol: String,
    pub market_data: Option<MarketData>,
}

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: CurrentPrice,
}

#[derive(Deserialize, Debug)]
pub struct CurrentPrice {
    pub btc: f64,
    pub eth: f64,
    pub eur: f64,
    pub jpy: f64,
    pub usd: f64,
}
