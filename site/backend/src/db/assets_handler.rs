use crate::db::db_handler;
use log::error;
use model::model::asset::{AssetData, LandAssetData, Price, TransactionData};
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::{query_as, FromRow, PgPool};

pub async fn get_asset_data_for_token_address_and_token_id(
    token_address: &String,
    token_id: &i32,
) -> Option<LandAssetData> {
    let pool = db_handler::open_connection().await;
    let transaction_data = get_all_transaction_data(&pool, token_address, token_id).await;

    let land_asset: Option<LandAssetDb> = match query_as(
        "select token_id, token_address, current_owner, created_on, name, tier,\
         solon, carbon, crypton, silicon, hydrogen, hyperion, landmark \
         from asset where token_address=$1 and token_id=$2",
    )
    .bind(token_address)
    .bind(token_id)
    .fetch_one(&pool)
    .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    let result = match land_asset {
        Some(res) => {
            fn get_last_owner_change(
                transaction_data: &Vec<TransactionData>,
                default: NaiveDateTime,
            ) -> NaiveDateTime {
                return match transaction_data.iter().find(|data| {
                    !data.event.contains("cancelled") && !data.event.contains("active")
                }) {
                    Some(data) => data.updated_on,
                    None => default,
                };
            }

            Some(LandAssetData {
                asset_data: AssetData {
                    token_id: res.token_id,
                    token_address: res.token_address,
                    current_owner: res.current_owner,
                    last_owner_change: get_last_owner_change(&transaction_data, res.created_on),
                },
                transaction_data,
                name: res.name,
                tier: res.tier,
                solon: res.solon,
                carbon: res.carbon,
                crypton: res.crypton,
                silicon: res.silicon,
                hydrogen: res.hydrogen,
                hyperion: res.hyperion,
                landmark: res.landmark,
            })
        }
        None => None,
    };

    db_handler::close_connection(pool).await;

    return result;
}

async fn get_all_transaction_data(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Vec<TransactionData> {
    let mut transfers =
        get_all_transfers_for_token_address_and_token_id(pool, token_address, token_id)
            .await
            .unwrap_or_default();
    let orders = get_all_order_data_for_token_address_and_token_id(pool, token_address, token_id)
        .await
        .unwrap_or_default();
    transfers.extend(orders);

    let mints = get_all_mint_data_for_token_address_and_token_id(pool, token_address, token_id)
        .await
        .unwrap_or_default();
    transfers.extend(mints);

    transfers.sort_by(|a, b| b.updated_on.cmp(&a.updated_on));

    transfers
}

async fn get_all_transfers_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Option<Vec<TransactionData>> {
    let result: Option<Vec<TransferDataDb>> = match query_as(
        "select transaction_id, wallet_from, wallet_to, created_on from transfer where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    match result {
        Some(res) => Some(res.into_iter().map(|t| t.into()).collect()),
        None => None,
    }
}

async fn get_all_order_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Option<Vec<TransactionData>> {
    let result: Option<Vec<OrderDataDb>> = match query_as(
        "select transaction_id, status, wallet_from, wallet_to, updated_on, buy_currency, buy_price, sell_price from order_data where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    match result {
        Some(res) => Some(res.into_iter().map(|t| t.into()).collect()),
        None => None,
    }
}

async fn get_all_mint_data_for_token_address_and_token_id(
    pool: &PgPool,
    token_address: &String,
    token_id: &i32,
) -> Option<Vec<TransactionData>> {
    let result: Option<Vec<MintDataDb>> = match query_as(
        "select transaction_id, wallet, currency, price, minted_on from mint where token_address=$1 and token_id=$2")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    match result {
        Some(res) => Some(res.into_iter().map(|mint| mint.into()).collect()),
        None => None,
    }
}

// https://github.com/launchbadge/sqlx/discussions/1886
// sqlx is not wasm compatible, so the dependency cannot be used in the `ui` module
#[derive(FromRow)]
struct LandAssetDb {
    token_id: i32,
    token_address: String,
    current_owner: String,
    created_on: NaiveDateTime,
    name: String,
    tier: i32,
    solon: i32,
    carbon: i32,
    crypton: i32,
    silicon: i32,
    hydrogen: i32,
    hyperion: i32,
    landmark: String,
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
        Self {
            id: transfer_data.transaction_id,
            wallet_from: transfer_data.wallet_from,
            wallet_to: transfer_data.wallet_to,
            event: "Transfer".to_string(),
            updated_on: transfer_data.created_on,
            price: None,
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
    sell_price: Option<Decimal>,
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
                price: if order_data.sell_price.is_some() {
                    order_data
                        .sell_price
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .unwrap()
                } else {
                    order_data.buy_price.to_string().parse::<f32>().unwrap()
                },
            }),
        }
    }
}

#[derive(FromRow)]
struct MintDataDb {
    transaction_id: i32,
    wallet: String,
    minted_on: NaiveDateTime,
    currency: String,
    price: Decimal,
}

impl From<MintDataDb> for TransactionData {
    fn from(mint_data_db: MintDataDb) -> Self {
        Self {
            id: Some(mint_data_db.transaction_id),
            wallet_from: "".to_string(),
            wallet_to: mint_data_db.wallet,
            event: "Mint".to_string(),
            updated_on: mint_data_db.minted_on,
            price: Some(Price {
                currency: mint_data_db.currency,
                price: mint_data_db.price.to_string().parse::<f32>().unwrap(),
            }),
        }
    }
}
