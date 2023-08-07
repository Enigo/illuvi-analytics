use crate::db::db_model::SingleTradeDb;
use log::error;
use model::model::price::Price;
use model::model::trade::SingleTrade;
use model::model::vitals::{TotalMintedBurnt, VitalsData, VitalsDataFloor};
use sqlx::types::Decimal;
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};
use std::collections::BTreeMap;

pub async fn get_all_vitals_for_token_address(
    pool: &Pool<Postgres>,
    token_address: &String,
) -> Option<VitalsData> {
    let total_assets = fetch_total_assets(pool, token_address).await;
    let floor_by_attribute = fetch_floor_data_by_attribute(pool, token_address).await;
    let trades_volume = fetch_trades_volume(pool, token_address).await;
    let last_trades = fetch_last_trades(pool, token_address).await;
    let minted_burnt_by_attribute = fetch_minted_burnt_by_attribute(pool, token_address).await;

    return Some(VitalsData {
        total_assets: total_assets.0,
        unique_holders: total_assets.1,
        floor_by_attribute,
        trades_volume,
        last_trades,
        minted_burnt_by_attribute,
    });
}

async fn fetch_total_assets(pool: &Pool<Postgres>, token_address: &String) -> (i64, i64) {
    let total_assets: (i64, i64) = match query(
        "select total, total_owners from asset_current_owner_mat_view where token_address=$1",
    )
    .bind(token_address)
    .fetch_one(pool)
    .await
    {
        Ok(result) => (result.get(0), result.get(1)),
        Err(e) => {
            error!("Error fetching data: {e}");
            (0, 0)
        }
    };
    total_assets
}

async fn fetch_floor_data_by_attribute(
    pool: &Pool<Postgres>,
    token_address: &String,
) -> BTreeMap<String, Vec<VitalsDataFloor>> {
    return match query_as::<_, FloorDataDb>(
        "select name, attribute, token_id, buy_price, buy_currency from floor_data_mat_by_attribute_view where token_address=$1")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => {
            let mut floor_data_by_attribute: BTreeMap<String, Vec<VitalsDataFloor>> = BTreeMap::new();
            for data in result {
                let trades_map = floor_data_by_attribute.entry(data.attribute.clone()).or_insert(Vec::new());
                trades_map.push(data.into());
            }
            floor_data_by_attribute
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            BTreeMap::new()
        }
    };
}

async fn fetch_trades_volume(pool: &Pool<Postgres>, token_address: &String) -> Vec<Price> {
    let trades_volume = match query_as::<_, PriceDb>(
        "select sum_eth, sum_usd from trade_volume_mat_view where token_address=$1",
    )
    .bind(token_address)
    .fetch_one(pool)
    .await
    {
        Ok(result) => {
            vec![
                Price {
                    price: f64::try_from(result.sum_eth).unwrap(),
                    currency: String::from("ETH"),
                },
                Price {
                    price: f64::try_from(result.sum_usd).unwrap(),
                    currency: String::from("USD"),
                },
            ]
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
    trades_volume
}

async fn fetch_last_trades(pool: &Pool<Postgres>, token_address: &String) -> Vec<SingleTrade> {
    let last_trades = match query_as::<_, SingleTradeDb>(
        // attribute is not needed for this data as of now
        "SELECT a.token_id, a.name AS attribute, a.name, round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
                od.buy_price, od.wallet_to, od.wallet_from, od.updated_on, od.transaction_id
                FROM asset a
                         JOIN order_data od ON a.token_id = od.token_id and a.token_address=od.token_address
                         JOIN coin_history ch ON ch.datestamp = od.updated_on::date AND od.buy_currency = ch.symbol
                WHERE od.status = 'filled' and a.token_address=$1
                order by od.updated_on desc
                limit 3;")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|trade| trade.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
    last_trades
}

async fn fetch_minted_burnt_by_attribute(
    pool: &Pool<Postgres>,
    token_address: &String,
) -> BTreeMap<String, Vec<TotalMintedBurnt>> {
    return match query_as::<_, TotalMintedBurntDb>(
        "select attribute, total_minted, total_burnt from total_minted_and_burnt_by_attribute_mat_view where token_address=$1")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => {
            let mut floor_data_by_attribute: BTreeMap<String, Vec<TotalMintedBurnt>> = BTreeMap::new();
            for data in result {
                let trades_map = floor_data_by_attribute.entry(data.attribute.clone()).or_insert(Vec::new());
                trades_map.push(data.into());
            }
            floor_data_by_attribute
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            BTreeMap::new()
        }
    };
}

#[derive(FromRow)]
struct FloorDataDb {
    pub attribute: String,
    pub token_id: i32,
    pub buy_price: Decimal,
    pub buy_currency: String,
    pub name: String,
}

impl From<FloorDataDb> for VitalsDataFloor {
    fn from(floor: FloorDataDb) -> Self {
        Self {
            token_id: floor.token_id,
            name: floor.name,
            price: Price {
                price: f64::try_from(floor.buy_price).unwrap(),
                currency: floor.buy_currency,
            },
        }
    }
}

#[derive(FromRow)]
struct PriceDb {
    sum_eth: Decimal,
    sum_usd: Decimal,
}

#[derive(FromRow)]
struct TotalMintedBurntDb {
    attribute: String,
    total_minted: i64,
    total_burnt: i64,
}
impl From<TotalMintedBurntDb> for TotalMintedBurnt {
    fn from(data: TotalMintedBurntDb) -> Self {
        Self {
            total_minted: data.total_minted,
            total_burnt: data.total_burnt,
        }
    }
}
