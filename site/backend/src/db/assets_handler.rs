use crate::db::db_handler;
use log::error;
use model::model::asset::{AssetData, LandAssetData, MintData, TransactionData};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query_as, FromRow, PgPool};

pub async fn get_asset_data_for_token_address_and_token_id(
    token_address: &String,
    token_id: &i32,
) -> Option<LandAssetData> {
    let pool = db_handler::open_connection().await;
    let transaction_data = get_all_transaction_data(&pool, token_address, token_id).await;

    let land_asset: Option<LandAssetDb> = match query_as(
        "select a.token_id, a.token_address, a.current_owner, a.updated_on, a.name, a.tier,\
         a.solon, a.carbon, a.crypton, a.silicon, a.hydrogen, a.hyperion, a.landmark, m.transaction_id, m.wallet, m.minted_on \
         from asset a join mint m on a.token_id=m.token_id where a.token_address=$1 and a.token_id=$2")
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
        Some(res) => Some(LandAssetData {
            asset_data: AssetData {
                token_id: res.token_id,
                token_address: res.token_address,
                current_owner: res.current_owner,
                last_owner_change: res.updated_on,
            },
            mint_data: MintData {
                transaction_id: res.transaction_id,
                wallet: res.wallet,
                minted_on: res.minted_on,
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
        }),
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
        "select order_id, status, wallet_from, wallet_to, updated_on from order_data where token_address=$1 and token_id=$2")
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

// https://github.com/launchbadge/sqlx/discussions/1886
// sqlx is not wasm compatible, so the dependency cannot be used in the `ui` module
#[derive(FromRow)]
struct LandAssetDb {
    token_id: i32,
    token_address: String,
    current_owner: String,
    updated_on: NaiveDateTime,
    name: String,
    tier: i32,
    solon: i32,
    carbon: i32,
    crypton: i32,
    silicon: i32,
    hydrogen: i32,
    hyperion: i32,
    landmark: String,
    transaction_id: i32,
    wallet: String,
    minted_on: NaiveDateTime,
}

#[derive(FromRow)]
struct TransferDataDb {
    transaction_id: i32,
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
        }
    }
}

#[derive(FromRow)]
struct OrderDataDb {
    order_id: i32,
    wallet_from: String,
    wallet_to: Option<String>,
    status: String,
    updated_on: NaiveDateTime,
}

impl From<OrderDataDb> for TransactionData {
    fn from(order_data: OrderDataDb) -> Self {
        Self {
            id: order_data.order_id,
            wallet_from: order_data.wallet_from,
            wallet_to: order_data.wallet_to.unwrap_or_default(),
            event: format!("{} {}", "Trade", order_data.status),
            updated_on: order_data.updated_on,
        }
    }
}
