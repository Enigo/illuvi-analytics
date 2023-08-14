use log::error;
use model::model::asset::{
    AccessoriesAssetData, AssetContentData, AssetData, CommonAssetData, D1skAssetData,
    IlluvitarAssetData, LandAssetData, TransactionData,
};
use model::model::price::Price;
use sqlx::types::{chrono::NaiveDateTime, Decimal};
use sqlx::{query, query_as, FromRow, PgPool, Pool, Postgres, Row};

const BURNED_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const BURNED: &str = "Burned";

const D1SK: &str = "0xc1f1da534e227489d617cd742481fd5a23f6a003";
const LAND: &str = "0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";
const ILLUVITAR: &str = "0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8";
const ACCESSORIES: &str = "0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24";

pub async fn get_asset_for_token_address_and_token_id(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
) -> Option<AssetData> {
    let common_asset_data = match query_as::<_, CommonAssetDb>(
        "select token_id, token_address, current_owner, metadata->>'name' as name, metadata->>'image_url' as image_url
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
    } else if token_address == ACCESSORIES {
        return get_accessories_asset(pool, token_address, token_id, common_asset_data.unwrap())
            .await;
    } else if token_address == ILLUVITAR {
        return get_illuvitar_asset(pool, token_address, token_id, common_asset_data.unwrap())
            .await;
    } else if token_address == LAND {
        return get_land_asset(pool, token_address, token_id, common_asset_data.unwrap()).await;
    }

    return None;
}

async fn get_d1sk_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    let content = match query_as::<_, AssetContentDb>(
        "select token_id, token_address, metadata->>'name' as name from asset where (metadata ->> 'Source Disk Id')::int4 = $1
            and metadata ->> 'Base Illuvitar Token Id' is null order by token_address, name")
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
                    content,
                }),
                land: None,
                accessories: None,
                illuvitar: None,
            })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

async fn get_accessories_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    let illuvitar = match query_as::<_, AssetContentDb>(
        "WITH acc_data AS (
            SELECT token_id, (metadata ->> 'Slot') AS slot
            FROM asset
            where (metadata ->> 'Slot') is not null and token_id=$1
        )
        SELECT iluv.token_id, iluv.token_address, iluv.metadata->>'name' as name
        FROM asset iluv
                 JOIN acc_data acc ON acc.token_id = (iluv.metadata ->> (acc.slot || ' Token Id'))::int4")
        .bind(token_id)
        .fetch_optional(pool)
        .await
    {
        Ok(result) => result.map(|value| value.into()),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    return match query_as::<_, AccessoriesAssetDb>(
        "select metadata->>'Tier' as tier, metadata->>'Stage' as stage, metadata->>'Slot' as slot,
         metadata->>'Source Disk Type' as source_disk_type, (metadata ->> 'Source Disk Id')::int4 as source_disk_id,
         metadata->>'Multiplier' as multiplier
         from asset where token_address=$1 and token_id=$2",
    )
        .bind(token_address)
        .bind(token_id)
        .fetch_one(pool)
        .await
    {
        Ok(result) => {
            Some(AssetData {
                accessories: Some(AccessoriesAssetData {
                    common_asset_data,
                    tier: result.tier,
                    stage: result.stage,
                    slot: result.slot,
                    source_token_address: D1SK.to_owned(),
                    source_disk_type: result.source_disk_type,
                    source_disk_id: result.source_disk_id,
                    multiplier: result.multiplier,
                    illuvitar,
                }),
                land: None,
                d1sk: None,
                illuvitar: None,
            })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

async fn get_illuvitar_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    let accessories: Vec<AssetContentData> = match query(
        "SELECT value::int4 as token_id, metadata ->> (replace(key, ' Token Id', '') || ' Name') as name
                     FROM asset CROSS JOIN LATERAL jsonb_each_text(metadata) AS m(key, value)
                     WHERE token_address=$1 and token_id=$2 and key LIKE '%Token Id'
                       AND key NOT LIKE '%Base Illuvitar Token Id%'")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            result.into_iter().map(|row| AssetContentData {
                token_id: row.get(0), token_address: ACCESSORIES.to_owned(), name: row.get(1)
            }).collect()
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };

    let accessorised_illuvitar_id: Option<i32> = match query(
        "select token_id from asset where (metadata ->> 'Base Illuvitar Token Id')::integer = $1",
    )
    .bind(token_id)
    .fetch_optional(pool)
    .await
    {
        Ok(result) => result.map(|value| value.get(0)),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    return match query_as::<_, IlluvitarAssetDb>(
        "select metadata->>'Set' as set, metadata->>'Line' as line, metadata->>'Tier' as tier, metadata->>'Wave' as wave,
         metadata->>'Stage' as stage, metadata->>'Class' as class, metadata->>'Affinity' as affinity,
         metadata->>'Expression' as expression, (metadata->>'Total Power')::int4 as total_power,
         metadata->>'Source Disk Type' as source_disk_type, (metadata->>'Source Disk Id')::int4 as source_disk_id, (metadata->>'Base Illuvitar Token Id')::int4 as origin_illuvitar_id
         from asset where token_address=$1 and token_id=$2",
    )
        .bind(token_address)
        .bind(token_id)
        .fetch_one(pool)
        .await
    {
        Ok(result) => {
            Some(AssetData {
                illuvitar: Some(IlluvitarAssetData {
                    common_asset_data,
                    set: result.set,
                    line: result.line,
                    tier: result.tier,
                    wave: result.wave,
                    stage: result.stage,
                    class: result.class,
                    affinity: result.affinity,
                    expression: result.expression,
                    total_power: result.total_power,
                    source_token_address: D1SK.to_owned(),
                    source_disk_type: result.source_disk_type,
                    source_disk_id: result.source_disk_id,
                    origin_illuvitar_id: result.origin_illuvitar_id,
                    accessorised_illuvitar_id,
                    accessories,
                }),
                land: None,
                d1sk: None,
                accessories: None,
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
                d1sk: None,
                accessories: None,
                illuvitar: None,
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
            case when od.status='active' then ch.datestamp=now()::date
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
struct AssetContentDb {
    token_id: i32,
    token_address: String,
    name: String,
}

impl From<AssetContentDb> for AssetContentData {
    fn from(data: AssetContentDb) -> Self {
        Self {
            token_id: data.token_id,
            token_address: data.token_address,
            name: data.name,
        }
    }
}

#[derive(FromRow)]
struct AccessoriesAssetDb {
    tier: String,
    stage: String,
    slot: String,
    source_disk_type: String,
    source_disk_id: i32,
    multiplier: String,
}

#[derive(FromRow)]
struct IlluvitarAssetDb {
    set: String,
    line: String,
    tier: String,
    wave: String,
    stage: String,
    class: String,
    affinity: String,
    expression: String,
    total_power: i32,
    source_disk_type: String,
    source_disk_id: i32,
    origin_illuvitar_id: Option<i32>,
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
