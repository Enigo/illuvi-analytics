use log::error;
use model::model::asset::{
    AssetData, CommonAssetData, D1skAssetData, LandAssetData, TransactionData,
};
use model::model::price::Price;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::{query_as, FromRow, PgPool, Pool, Postgres};

const BURNED_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const BURNED: &str = "Burned";
const D1SK: &str = "0xc1f1da534e227489d617cd742481fd5a23f6a003";
const LAND: &str = "0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";

pub async fn get_asset_for_token_address_and_token_id(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
) -> Option<AssetData> {
    let common_asset_data = match query_as::<_, CommonAssetDb>(
        "select token_id, token_address, current_owner, name, metadata->>'image_url' as image_url
         from asset where token_address=$1 and token_id=$2",
    )
    .bind(token_address)
    .bind(token_id)
    .fetch_one(pool)
    .await
    {
        Ok(result) => {
            let burned = result.current_owner == BURNED_ADDRESS;

            Some(CommonAssetData {
                token_id: result.token_id,
                token_address: result.token_address,
                current_owner: if burned {
                    BURNED.to_string()
                } else {
                    result.current_owner
                },
                burned,
                name: result.name,
                image_url: result.image_url,
            })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    if common_asset_data.is_none() {
        return None;
    }

    if token_address == D1SK {
        return get_d1sk_asset(pool, token_address, token_id, common_asset_data.unwrap()).await;
    }

    return get_land_asset(pool, token_address, token_id, common_asset_data.unwrap()).await;
}

async fn get_d1sk_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    return match query_as::<_, D1skAssetDb>(
        "select (metadata->>'Alpha')::bool as alpha, metadata->>'Wave' as wave, metadata->>'Set' as set
         from asset where token_address=$1 and token_id=$2",
    )
        .bind(token_address)
        .bind(token_id)
        .fetch_one(pool)
        .await
    {
        Ok(result) => {
            Some(AssetData {
                d1sk: Some(D1skAssetData {
                    common_asset_data,
                    alpha: result.alpha,
                    wave: result.wave,
                    set: result.set,
                }),
                land: None,
            })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

async fn get_land_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    return match query_as::<_, LandAssetDb>(
        "select metadata->>'tier' as tier, metadata->>'solon' as solon, metadata->>'carbon' as carbon, metadata->>'crypton' as crypton,
         metadata->>'silicon' as silicon, metadata->>'hydrogen' as hydrogen, metadata->>'hyperion' as hyperion, metadata->>'landmark' as landmark
         from asset where token_address=$1 and token_id=$2",
    )
        .bind(token_address)
        .bind(token_id)
        .fetch_one(pool)
        .await
    {
        Ok(result) => {
            Some(AssetData {
                land: Some(LandAssetData {
                    common_asset_data,
                    tier: result.tier,
                    solon: result.solon,
                    carbon: result.carbon,
                    crypton: result.crypton,
                    silicon: result.silicon,
                    hydrogen: result.hydrogen,
                    hyperion: result.hyperion,
                    landmark: result.landmark,
                }),
                d1sk: None
            })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

pub async fn get_events_for_token_address_and_token_id(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
) -> Option<Vec<TransactionData>> {
    let mut transfers =
        get_all_transfers_for_token_address_and_token_id(pool, token_address, token_id).await;
    let orders =
        get_all_order_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    transfers.extend(orders);

    let mints =
        get_all_mint_data_for_token_address_and_token_id(pool, token_address, token_id).await;
    transfers.extend(mints);

    transfers.sort_by(|a, b| b.updated_on.cmp(&a.updated_on));

    Some(transfers)
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
        "select transaction_id, status, wallet_from, wallet_to, updated_on, buy_currency, buy_price from order_data where token_address=$1 and token_id=$2")
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
        "select transaction_id, wallet, currency, price, minted_on from mint where token_address=$1 and token_id=$2")
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

// https://github.com/launchbadge/sqlx/discussions/1886
// sqlx is not wasm compatible, so the dependency cannot be used in the `ui` module
#[derive(FromRow)]
struct CommonAssetDb {
    token_id: i32,
    token_address: String,
    current_owner: String,
    name: String,
    image_url: String,
}

#[derive(FromRow)]
struct LandAssetDb {
    tier: String,
    solon: String,
    carbon: String,
    crypton: String,
    silicon: String,
    hydrogen: String,
    hyperion: String,
    landmark: String,
}

#[derive(FromRow)]
struct D1skAssetDb {
    alpha: bool,
    wave: String,
    set: String,
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
            wallet_to: transfer_data.wallet_to.clone(),
            event: if transfer_data.wallet_to == BURNED_ADDRESS {
                BURNED.to_string()
            } else {
                "Transfer".to_string()
            },
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
                price: f64::try_from(mint_data_db.price).unwrap(),
            }),
        }
    }
}
