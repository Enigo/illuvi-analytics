use log::error;
use model::model::asset::TransactionData;
use model::model::price::Price;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::{query_as, FromRow, PgPool, Pool, Postgres};

const BURNED_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const BURNED: &str = "Burned";

pub async fn get_events_for_token_address_and_token_id(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
) -> Option<Vec<TransactionData>> {
    let mut events =
        get_all_transfers_for_token_address_and_token_id(pool, token_address, token_id).await;
    let orders =
        get_all_order_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    events.extend(orders);

    let mints =
        get_all_mint_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    events.extend(mints);

    let deposits =
        get_all_deposit_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    events.extend(deposits);

    let withdrawals =
        get_all_withdrawal_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    events.extend(withdrawals);

    events.sort_by(|a, b| b.updated_on.cmp(&a.updated_on));

    Some(events)
}

async fn get_all_transfers_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    return match query_as::<_, TransferDataDb>(
        "select transaction_id, wallet_from, wallet_to, created_on from transfer where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter().map(|t| t.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_all_order_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    return match query_as::<_, OrderDataDb>(
        "select od.transaction_id, od.status, od.wallet_from, od.wallet_to, od.updated_on, od.buy_currency, od.buy_price, round((od.buy_price * ch.usd), 2) as usd_price
         from order_data od join coin_history ch on
            case when od.status='active'
                then ch.datestamp=(SELECT MAX(datestamp) FROM coin_history)
                else ch.datestamp = od.updated_on::date
            end
            and od.buy_currency = ch.symbol
         where od.token_address=$1 and od.token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter().map(|t| t.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_all_mint_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    return match query_as::<_, MintDataDb>(
        "select m.transaction_id, m.wallet, m.currency, m.price, m.minted_on, round((m.price * ch.usd), 2) as usd_price from mint m
         join coin_history ch on ch.datestamp = m.minted_on::date and (m.currency is null OR ch.symbol = m.currency)
         where token_address=$1 and token_id=$2 limit 1") // limit 1 for the case when currency is null
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter().map(|mint| mint.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_all_deposit_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    return match query_as::<_, DepositDataDb>(
        "select transaction_id, wallet, created_on from deposit where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter().map(|mint| mint.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_all_withdrawal_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    return match query_as::<_, WithdrawalDataDb>(
        "select transaction_id, wallet, created_on from withdrawal where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter().map(|mint| mint.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

#[derive(FromRow)]
struct TransferDataDb {
    transaction_id: Option<i32>,
    wallet_from: String,
    wallet_to: String,
    created_on: NaiveDateTime,
}

impl From<TransferDataDb> for TransactionData {
    fn from(transfer_data: TransferDataDb) -> Self {
        let wallet_to = transfer_data.wallet_to;
        Self {
            id: transfer_data.transaction_id,
            wallet_from: transfer_data.wallet_from,
            wallet_to: if wallet_to == BURNED_ADDRESS {
                "".to_string()
            } else {
                wallet_to.clone()
            },
            event: if wallet_to == BURNED_ADDRESS {
                BURNED.to_string()
            } else {
                "Transfer".to_string()
            },
            updated_on: transfer_data.created_on,
            price: None,
            usd_price: None,
        }
    }
}

#[derive(FromRow)]
struct OrderDataDb {
    transaction_id: Option<i32>,
    wallet_from: String,
    wallet_to: Option<String>,
    status: String,
    updated_on: NaiveDateTime,
    buy_currency: String,
    buy_price: Decimal,
    usd_price: Decimal,
}

impl From<OrderDataDb> for TransactionData {
    fn from(order_data: OrderDataDb) -> Self {
        Self {
            id: order_data.transaction_id,
            wallet_from: order_data.wallet_from,
            wallet_to: order_data.wallet_to.unwrap_or_default(),
            event: format!("{} {}", "Trade", order_data.status),
            updated_on: order_data.updated_on,
            price: Some(Price {
                currency: order_data.buy_currency,
                price: f64::try_from(order_data.buy_price).unwrap(),
            }),
            usd_price: Some(Price {
                currency: "USD".to_owned(),
                price: f64::try_from(order_data.usd_price).unwrap(),
            }),
        }
    }
}

#[derive(FromRow)]
struct MintDataDb {
    transaction_id: i32,
    wallet: String,
    minted_on: NaiveDateTime,
    currency: Option<String>,
    price: Option<Decimal>,
    usd_price: Option<Decimal>,
}

impl From<MintDataDb> for TransactionData {
    fn from(mint_data_db: MintDataDb) -> Self {
        let mut price = None;
        let mut usd_price = None;
        if mint_data_db.currency.is_some() {
            price = Some(Price {
                currency: mint_data_db.currency.unwrap(),
                price: f64::try_from(mint_data_db.price.unwrap()).unwrap(),
            });

            usd_price = Some(Price {
                currency: "USD".to_owned(),
                price: f64::try_from(mint_data_db.usd_price.unwrap()).unwrap(),
            })
        }

        Self {
            id: Some(mint_data_db.transaction_id),
            wallet_from: "".to_string(),
            wallet_to: mint_data_db.wallet,
            event: "Mint".to_string(),
            updated_on: mint_data_db.minted_on,
            price,
            usd_price,
        }
    }
}

#[derive(FromRow)]
struct DepositDataDb {
    transaction_id: i32,
    wallet: String,
    created_on: NaiveDateTime,
}

impl From<DepositDataDb> for TransactionData {
    fn from(data: DepositDataDb) -> Self {
        Self {
            id: Some(data.transaction_id),
            wallet_from: "".to_string(),
            wallet_to: data.wallet,
            event: "Deposit".to_string(),
            updated_on: data.created_on,
            price: None,
            usd_price: None,
        }
    }
}

#[derive(FromRow)]
struct WithdrawalDataDb {
    transaction_id: i32,
    wallet: String,
    created_on: NaiveDateTime,
}

impl From<WithdrawalDataDb> for TransactionData {
    fn from(data: WithdrawalDataDb) -> Self {
        Self {
            id: Some(data.transaction_id),
            wallet_from: data.wallet,
            wallet_to: "".to_string(),
            event: "Withdrawal".to_string(),
            updated_on: data.created_on,
            price: None,
            usd_price: None,
        }
    }
}
