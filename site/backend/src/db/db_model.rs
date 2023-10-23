use model::model::asset::AssetContentData;
use model::model::price::Price;
use model::model::transaction::SingleTransaction;
use model::model::transaction::TransactionData;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::FromRow;

#[derive(FromRow, Clone)]
pub struct SingleTransactionDb {
    pub attribute: String,
    pub token_id: i32,
    pub name: String,
    pub image_url: String,
    pub sum_usd: Decimal,
    pub buy_currency: String,
    pub buy_price: Decimal,
    pub updated_on: NaiveDateTime,
    pub transaction_id: Option<i32>,
}

impl From<SingleTransactionDb> for SingleTransaction {
    fn from(transaction: SingleTransactionDb) -> Self {
        Self {
            token_id: transaction.token_id,
            name: transaction.name.clone(),
            image_url: transaction.image_url.clone(),
            usd_price: Price {
                price: transaction.sum_usd.to_string().parse::<f64>().unwrap(),
                currency: String::from("USD"),
            },
            buy_price: Price {
                price: transaction.buy_price.to_string().parse::<f64>().unwrap(),
                currency: transaction.buy_currency.clone(),
            },
            updated_on: transaction.updated_on,
            transaction_id: transaction.transaction_id,
        }
    }
}

#[derive(FromRow)]
pub struct TransactionDataDb {
    transaction_id: Option<i32>,
    wallet_from: Option<String>,
    wallet_to: Option<String>,
    event: String,
    timestamp: NaiveDateTime,
    currency: Option<String>,
    price: Option<Decimal>,
    usd_price: Option<Decimal>,

    token_address: Option<String>,
    token_id: Option<i32>,
    name: Option<String>,
    image_url: Option<String>,
}

impl From<TransactionDataDb> for TransactionData {
    fn from(event_data: TransactionDataDb) -> Self {
        let price = event_data.currency.map(|value| Price {
            currency: value,
            price: f64::try_from(event_data.price.unwrap()).unwrap(),
        });
        let usd_price = event_data.usd_price.map(|value| Price {
            currency: "USD".to_owned(),
            price: f64::try_from(value).unwrap(),
        });

        let asset_content = event_data
            .token_address
            .map(|token_address| AssetContentData {
                token_id: event_data.token_id.unwrap(),
                token_address,
                name: event_data.name.unwrap(),
                image_url: event_data.image_url.unwrap(),
            });

        Self {
            id: event_data.transaction_id,
            wallet_from: event_data.wallet_from.unwrap_or_default(),
            wallet_to: event_data.wallet_to.unwrap_or_default(),
            event: event_data.event,
            updated_on: event_data.timestamp,
            price,
            usd_price,
            asset_content,
        }
    }
}
