use crate::db::db_model::SingleTransactionDb;
use log::error;
use model::model::asset::{
    AccessoriesAssetData, AssetContentData, AssetData, CommonAssetData, CommonOrderData,
    D1skAssetData, IlluvitarAssetData, LandAssetData,
};
use model::model::transaction::SingleTransaction;
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};

const BURNED_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

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
                current_owner: result.current_owner,
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
        "select token_id, token_address, metadata->>'name' as name, metadata->>'image_url' as image_url
         from asset where (metadata ->> 'Source Disk Id')::int4 = $1
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

    let last_filled_orders = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol and od.updated_on::date = ch.datestamp
        where a.token_address = $1 and od.status = 'filled'
        and a.attribute = ca.attribute
        order by updated_on desc")
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

    let common_order_data = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol
        where a.token_address = $1 and od.status = 'active'
        and a.attribute = ca.attribute and ch.datestamp = (select max(datestamp) from coin_history)
        order by sum_usd, buy_price, token_id")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            let active_orders: Vec<SingleTransaction> = result.into_iter().map(|t| t.into()).collect();
            if active_orders.is_empty() {
                None
            } else {
                let listed_index = active_orders.iter().position(|item| &item.token_id == token_id);

                Some(CommonOrderData {
                    active_orders: active_orders.len() as i64,
                    total_filled_orders: last_filled_orders.len() as i64,
                    listed_index: listed_index.map(|index| (index + 1) as i64),
                    last_active_orders: active_orders.into_iter().take(3).collect(),
                    last_filled_orders: last_filled_orders.into_iter().take(3).collect()
                })
            }
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            None
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
                    common_order_data
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
        SELECT iluv.token_id, iluv.token_address, iluv.metadata->>'name' as name, iluv.metadata->>'image_url' as image_url
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

    let d1sk = get_source_d1sk(pool, token_address, token_id).await;

    let last_filled_orders = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol and od.updated_on::date = ch.datestamp
        where a.token_address = $1 and od.status = 'filled'
        and a.metadata->>'name' = ca.metadata->>'name'
        order by updated_on desc")
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

    let common_order_data = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol
        where a.token_address = $1 and od.status = 'active'
        and a.metadata->>'name' = ca.metadata->>'name' and ch.datestamp = (select max(datestamp) from coin_history)
        order by sum_usd, buy_price, token_id")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            let active_orders: Vec<SingleTransaction> = result.into_iter().map(|t| t.into()).collect();
            if active_orders.is_empty() {
                None
            } else {
                let listed_index = active_orders.iter().position(|item| &item.token_id == token_id);

                Some(CommonOrderData {
                    active_orders: active_orders.len() as i64,
                    total_filled_orders: last_filled_orders.len() as i64,
                    listed_index: listed_index.map(|index| (index + 1) as i64),
                    last_active_orders: active_orders.into_iter().take(3).collect(),
                    last_filled_orders: last_filled_orders.into_iter().take(3).collect()
                })
            }
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    return match query_as::<_, AccessoriesAssetDb>(
        "select metadata->>'Tier' as tier, metadata->>'Stage' as stage, metadata->>'Slot' as slot,
         metadata->>'Multiplier' as multiplier
         from asset where token_address=$1 and token_id=$2",
    )
    .bind(token_address)
    .bind(token_id)
    .fetch_one(pool)
    .await
    {
        Ok(result) => Some(AssetData {
            accessories: Some(AccessoriesAssetData {
                common_asset_data,
                common_order_data,
                tier: result.tier,
                stage: result.stage,
                slot: result.slot,
                multiplier: result.multiplier,
                d1sk,
                illuvitar,
            }),
            land: None,
            d1sk: None,
            illuvitar: None,
        }),
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
        "SELECT value::int4 as token_id, metadata ->> (replace(key, ' Token Id', '') || ' Name') as name,
                        metadata->>(replace(key, ' Token Id', '') || ' Image Url') as image_url
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
                token_id: row.get(0), token_address: ACCESSORIES.to_owned(), name: row.get(1), image_url: row.get(2)
            }).collect()
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };

    let accessorised_illuvitar = match query_as::<_, AssetContentDb>(
        "select token_id, token_address, metadata->>'name' as name, metadata->>'image_url' as image_url
         from asset where (metadata ->> 'Base Illuvitar Token Id')::integer = $1",
    )
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

    let origin_illuvitar = match query_as::<_, AssetContentDb>(
        "WITH acc_data AS (
            SELECT token_address, (metadata ->> 'Base Illuvitar Token Id') AS origin_illuvitar_id
            FROM asset
            where token_address=$1 and token_id=$2
        )
        select a.token_id, a.token_address, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url
        from asset a
                 join acc_data acc ON (acc.origin_illuvitar_id)::int4 = a.token_id and acc.token_address = a.token_address
        where a.token_id = (acc.origin_illuvitar_id)::int4",
    )
    .bind(token_address)
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

    let last_filled_orders = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol and od.updated_on::date = ch.datestamp
        where a.token_address = $1 and od.status = 'filled'
        and a.attribute = ca.attribute and a.metadata->>'Line' = ca.metadata->>'Line' and a.metadata->>'Finish' = ca.metadata->>'Finish' and a.metadata->>'Alpha' = ca.metadata->>'Alpha'
            and a.metadata->>'Stage' = ca.metadata->>'Stage'
        order by updated_on desc")
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

    let common_order_data = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol
        where a.token_address = $1 and od.status = 'active'
        and a.attribute = ca.attribute and a.metadata->>'Line' = ca.metadata->>'Line' and a.metadata->>'Finish' = ca.metadata->>'Finish' and a.metadata->>'Alpha' = ca.metadata->>'Alpha'
            and a.metadata->>'Stage' = ca.metadata->>'Stage' and ch.datestamp = (select max(datestamp) from coin_history)
        order by sum_usd, buy_price, token_id")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            let active_orders: Vec<SingleTransaction> = result.into_iter().map(|t| t.into()).collect();
            if active_orders.is_empty() {
                None
            } else {
                let listed_index = active_orders.iter().position(|item| &item.token_id == token_id);

                Some(CommonOrderData {
                    active_orders: active_orders.len() as i64,
                    total_filled_orders: last_filled_orders.len() as i64,
                    listed_index: listed_index.map(|index| (index + 1) as i64),
                    last_active_orders: active_orders.into_iter().take(3).collect(),
                    last_filled_orders: last_filled_orders.into_iter().take(3).collect()
                })
            }
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    let d1sk = get_source_d1sk(&pool, &token_address, &token_id).await;

    return match query_as::<_, IlluvitarAssetDb>(
        "select metadata->>'Set' as set, metadata->>'Line' as line, metadata->>'Tier' as tier, metadata->>'Wave' as wave,
         metadata->>'Stage' as stage, metadata->>'Class' as class, metadata->>'Affinity' as affinity,
         metadata->>'Expression' as expression, (metadata->>'Total Power')::int4 as total_power
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
                    common_order_data,
                    set: result.set,
                    line: result.line,
                    tier: result.tier,
                    wave: result.wave,
                    stage: result.stage,
                    class: result.class,
                    affinity: result.affinity,
                    expression: result.expression,
                    total_power: result.total_power,
                    d1sk,
                    origin_illuvitar,
                    accessorised_illuvitar,
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

async fn get_source_d1sk(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
) -> Option<AssetContentData> {
    let d1sk = match query_as::<_, AssetContentDb>(
        "WITH acc_data AS (
            SELECT (metadata ->> 'Source Disk Id') AS source_disk_id
            FROM asset
            where token_address=$1 and token_id=$2
        )
        select a.token_id, a.token_address, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url
        from asset a
                 join acc_data acc ON (acc.source_disk_id)::int4 = a.token_id
        where a.token_id = (acc.source_disk_id)::int4 and a.token_address=$3")
        .bind(token_address)
        .bind(token_id)
        .bind(D1SK)
        .fetch_optional(pool)
        .await
    {
        Ok(result) => result.map(|value| value.into()),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
    d1sk
}

async fn get_land_asset(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    common_asset_data: CommonAssetData,
) -> Option<AssetData> {
    let last_filled_orders = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol and od.updated_on::date = ch.datestamp
        where a.token_address = $1 and od.status = 'filled'
        and a.attribute = ca.attribute and a.metadata->>'region' = ca.metadata->>'region'
        order by updated_on desc")
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

    let common_order_data = match query_as::<_, SingleTransactionDb>(
        "with current_asset as (select * from asset where token_address=$1 and token_id=$2)
        select a.token_id, a.attribute, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url,
               round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
               od.buy_price, od.updated_on, od.transaction_id from order_data od
            join asset a on od.token_address = a.token_address and od.token_id = a.token_id
            join current_asset ca on ca.token_address = a.token_address
            join coin_history ch on od.buy_currency = ch.symbol
        where a.token_address = $1 and od.status = 'active'
        and a.attribute = ca.attribute and a.metadata->>'region' = ca.metadata->>'region' and ch.datestamp = (select max(datestamp) from coin_history)
        order by sum_usd, buy_price, token_id")
        .bind(token_address)
        .bind(token_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            let active_orders: Vec<SingleTransaction> = result.into_iter().map(|t| t.into()).collect();
            if active_orders.is_empty() {
                None
            } else {
                let listed_index = active_orders.iter().position(|item| &item.token_id == token_id);

                Some(CommonOrderData {
                    active_orders: active_orders.len() as i64,
                    total_filled_orders: last_filled_orders.len() as i64,
                    listed_index: listed_index.map(|index| (index + 1) as i64),
                    last_active_orders: active_orders.into_iter().take(3).collect(),
                    last_filled_orders: last_filled_orders.into_iter().take(3).collect()
                })
            }
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

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
                    common_order_data,
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
    image_url: String,
}

impl From<AssetContentDb> for AssetContentData {
    fn from(data: AssetContentDb) -> Self {
        Self {
            token_id: data.token_id,
            token_address: data.token_address,
            name: data.name,
            image_url: data.image_url,
        }
    }
}

#[derive(FromRow)]
struct AccessoriesAssetDb {
    tier: String,
    stage: String,
    slot: String,
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
}
