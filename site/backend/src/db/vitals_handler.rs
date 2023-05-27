use crate::db::db_model::SingleTradeDb;
use log::error;
use model::model::price::Price;
use model::model::vitals::{VitalsData, VitalsDataFloor};
use sqlx::types::Decimal;
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};

pub async fn get_all_vitals_for_token_address(
    pool: &Pool<Postgres>,
    token_address: &String,
) -> Option<VitalsData> {
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

    let floor_data = match query_as::<_, FloorDataDb>(
        "select tier, name, token_id, buy_price, buy_currency
            from (
                     select a.tier, a.name, o.token_id, o.buy_price, o.buy_currency,
                            row_number() over (partition by a.tier, o.buy_currency order by o.buy_price) as rn
                     from asset a
                              inner join (
                         select token_id, buy_currency, min(buy_price) as buy_price
                         from order_data
                         where status = 'active' and token_address=$1
                         group by token_id, buy_currency
                     ) o on a.token_id = o.token_id
                 ) t
            where rn = 1
            order by tier, buy_price"
    )
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|order| order.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };

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

    let last_trades = match query_as::<_, SingleTradeDb>(
        "SELECT a.token_id, a.tier, a.name, round((od.buy_price * ch.usd), 2) AS sum_usd, od.buy_currency,
                od.buy_price, od.wallet_to, od.wallet_from, od.updated_on, od.transaction_id
                FROM asset a
                         JOIN order_data od ON a.token_id = od.token_id
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

    return Some(VitalsData {
        total_assets: total_assets.0,
        unique_holders: total_assets.1,
        floor: floor_data,
        trades_volume,
        last_trades,
    });
}

#[derive(FromRow)]
struct FloorDataDb {
    pub tier: i32,
    pub token_id: i32,
    pub buy_price: Decimal,
    pub buy_currency: String,
    pub name: String,
}

impl From<FloorDataDb> for VitalsDataFloor {
    fn from(floor: FloorDataDb) -> Self {
        Self {
            tier: floor.tier,
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
