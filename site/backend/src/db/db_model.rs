use model::model::price::Price;
use model::model::trade::SingleTrade;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct SingleTradeDb {
    pub attribute: String,
    pub token_id: i32,
    pub name: String,
    pub image_url: String,
    pub sum_usd: Decimal,
    pub buy_currency: String,
    pub buy_price: Decimal,
    pub updated_on: NaiveDateTime,
    pub transaction_id: i32,
}

impl From<SingleTradeDb> for SingleTrade {
    fn from(trade: SingleTradeDb) -> Self {
        Self {
            token_id: trade.token_id,
            name: trade.name.clone(),
            image_url: trade.image_url.clone(),
            usd_price: Price {
                price: trade.sum_usd.to_string().parse::<f64>().unwrap(),
                currency: String::from("USD"),
            },
            buy_price: Price {
                price: trade.buy_price.to_string().parse::<f64>().unwrap(),
                currency: trade.buy_currency.clone(),
            },
            updated_on: trade.updated_on,
            transaction_id: trade.transaction_id,
        }
    }
}
