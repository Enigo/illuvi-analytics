use model::model::price::Price;
use model::model::trade::SingleTrade;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct SingleTradeDb {
    pub tier: i32,
    pub token_id: i32,
    pub name: String,
    pub sum_usd: Decimal,
    pub buy_currency: String,
    pub sell_price: Decimal,
    pub wallet_to: String,
    pub wallet_from: String,
    pub updated_on: NaiveDateTime,
    pub transaction_id: i32,
}

impl From<SingleTradeDb> for SingleTrade {
    fn from(trade: SingleTradeDb) -> Self {
        Self {
            token_id: trade.token_id,
            name: trade.name.clone(),
            usd_price: Price {
                price: trade.sum_usd.to_string().parse::<f64>().unwrap(),
                currency: String::from("USD"),
            },
            sell_price: Price {
                price: trade.sell_price.to_string().parse::<f64>().unwrap(),
                currency: trade.buy_currency.clone(),
            },
            wallet_to: trade.wallet_to.clone(),
            wallet_from: trade.wallet_from.clone(),
            updated_on: trade.updated_on,
            transaction_id: trade.transaction_id,
        }
    }
}
