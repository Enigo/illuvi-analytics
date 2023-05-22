use crate::db::db_handler;
use crate::db::db_model::SingleTradeDb;
use log::error;
use model::model::price::Price;
use model::model::stats::{
    StatsData, StatsDataMostEventForToken, StatsDataMostEventForWallet, StatsDataTotal,
    StatsDataTotalOrder, StatsDataTradesVolume,
};
use model::model::trade::SingleTrade;
use sqlx::types::Decimal;
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};
use std::collections::BTreeMap;

pub async fn get_all_stats_for_token_address(token_address: &String) -> Option<StatsData> {
    let pool = db_handler::open_connection().await;

    let assets = fetch_assets(token_address, &pool).await;
    let transfers = fetch_transfers(token_address, &pool).await;
    let total_trades = fetch_total_trades(token_address, &pool).await;
    let trades_volume = fetch_trades_volume(token_address, &pool).await;
    let most_transferred_token = fetch_most_transferred_token(token_address, &pool).await;
    let most_traded_token = fetch_most_traded_token(token_address, &pool).await;
    let most_traded_wallet = fetch_most_traded_wallet(token_address, &pool).await;
    let cheapest_and_most_expensive_trades_by_tier =
        fetch_cheapest_and_most_expensive_trades_by_tier(token_address, &pool).await;

    db_handler::close_connection(pool).await;

    return Some(StatsData {
        total: StatsDataTotal {
            assets,
            transfers,
            trades: total_trades.0,
        },
        trades_by_status: total_trades.1,
        trades_volume,
        most_transferred_token,
        most_traded_token,
        most_traded_wallet,
        cheapest_and_most_expensive_trades_by_tier,
    });
}

async fn fetch_assets(token_address: &String, pool: &Pool<Postgres>) -> i64 {
    return match query("select count(token_id) from mint where token_address=$1")
        .bind(token_address)
        .fetch_one(pool)
        .await
    {
        Ok(result) => result.get(0),
        Err(e) => {
            error!("Error fetching data: {e}");
            0
        }
    };
}

async fn fetch_transfers(token_address: &String, pool: &Pool<Postgres>) -> i64 {
    return match query("select count(token_id) from transfer where token_address=$1")
        .bind(token_address)
        .fetch_one(pool)
        .await
    {
        Ok(result) => result.get(0),
        Err(e) => {
            error!("Error fetching data: {e}");
            0
        }
    };
}

async fn fetch_total_trades(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> (i64, BTreeMap<String, Vec<StatsDataTotalOrder>>) {
    return match query_as::<_, OrderDb>(
        "select count(*), status, buy_currency from order_data where token_address=$1 group by 2, 3 order by 2, 1, 3")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => {
            let mut total_orders = 0;
            let mut orders_by_status_currency: BTreeMap<String, Vec<StatsDataTotalOrder>> = BTreeMap::new();
            for order in &result {
                let currency_map = orders_by_status_currency.entry(order.status.clone()).or_insert(Vec::new());
                currency_map.push(StatsDataTotalOrder { buy_currency: order.buy_currency.clone(), count: order.count });
                total_orders += order.count;
            }

            (total_orders, orders_by_status_currency)
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            (0, BTreeMap::new())
        }
    };
}

async fn fetch_trades_volume(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> Vec<StatsDataTradesVolume> {
    return match query_as::<_, StatsDataTradesVolumeDb>(
        "select count(*) as total_trades,
         round(SUM(od.sell_price), 6) as total_in_buy_currency,
         od.buy_currency,
         round(SUM(od.sell_price * ch.btc), 6) as total_btc,
         round(SUM(od.sell_price * ch.eth), 6) as total_eth,
         round(SUM(od.sell_price * ch.usd), 6) as total_usd,
         round(SUM(od.sell_price * ch.eur), 6) as total_eur,
         round(SUM(od.sell_price * ch.jpy), 6) as total_jpy
          from order_data od join coin_history ch on od.buy_currency = ch.symbol AND ch.datestamp = od.updated_on::DATE
                             where od.status='filled' and od.token_address=$1
                             group by od.buy_currency
                             order by total_usd desc;")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|volume| StatsDataTradesVolume {
            total_trades: volume.total_trades,
            total_in_buy_currency: Price { price: f64::try_from(volume.total_in_buy_currency).unwrap(), currency: volume.buy_currency },
            totals_in_other_currency: vec![
                Price { price: f64::try_from(volume.total_btc).unwrap(), currency: String::from("BTC") },
                Price { price: f64::try_from(volume.total_eth).unwrap(), currency: String::from("ETH") },
                Price { price: f64::try_from(volume.total_usd).unwrap(), currency: String::from("USD") },
                Price { price: f64::try_from(volume.total_eur).unwrap(), currency: String::from("EUR") },
                Price { price: f64::try_from(volume.total_jpy).unwrap(), currency: String::from("JPY") },
            ],
        }).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn fetch_most_transferred_token(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> Vec<StatsDataMostEventForToken> {
    return match query_as::<_, (i32, i64)>(
        "select token_id, total from (select token_id, count(*) as total from transfer where token_address=$1 group by token_id) as subquery
            where total = (select max(total)
                from (select count(*) as total from transfer where token_address=$1 group by token_id) as counts);")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|volume| StatsDataMostEventForToken {
            token_id: volume.0,
            count: volume.1,

        }).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn fetch_most_traded_token(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> Vec<StatsDataMostEventForToken> {
    return match query_as::<_, (i32, i64)>(
        "select token_id, total from (select token_id, count(*) as total from order_data
                                                                where token_address=$1 and status='filled' group by token_id) as subquery
                        where total = (select max(total)
                                from (select count(*) as total from order_data where token_address=$1 and status='filled' group by token_id) as counts);")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|volume| StatsDataMostEventForToken {
            token_id: volume.0,
            count: volume.1,

        }).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn fetch_most_traded_wallet(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> Vec<StatsDataMostEventForWallet> {
    return match query_as::<_, (String, i64)>(
        "select wallet_to, total from (select wallet_to, count(*) as total from order_data
                             where token_address=$1 and status='filled' group by wallet_to) as subquery
               where total = (select max(total)
               from (select count(*) as total from order_data where token_address=$1 and status='filled' group by wallet_to) as counts);")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => result.into_iter().map(|volume| StatsDataMostEventForWallet {
            wallet: volume.0,
            count: volume.1,
        }).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn fetch_cheapest_and_most_expensive_trades_by_tier(
    token_address: &String,
    pool: &Pool<Postgres>,
) -> BTreeMap<i32, Vec<SingleTrade>> {
    return match query_as::<_, SingleTradeDb>(
        "SELECT tier, token_id, name, sum_usd,  buy_currency, sell_price, wallet_to, wallet_from, updated_on, transaction_id
            FROM (
                     SELECT a.token_id, a.tier, a.name, round((od.sell_price * ch.usd), 2) AS sum_usd, od.buy_currency, od.sell_price, od.wallet_to, od.wallet_from, od.updated_on, od.transaction_id,
                            ROW_NUMBER() OVER (PARTITION BY a.tier ORDER BY (od.sell_price * ch.usd) DESC) AS highest_rn,
                            ROW_NUMBER() OVER (PARTITION BY a.tier ORDER BY (od.sell_price * ch.usd)) AS lowest_rn
                     FROM asset a
                              JOIN order_data od ON a.token_id = od.token_id
                              JOIN coin_history ch ON ch.datestamp = od.updated_on::date AND od.buy_currency = ch.symbol
                     WHERE od.status = 'filled' and od.token_address=$1
                 ) subquery
            WHERE highest_rn = 1 OR lowest_rn = 1
            ORDER BY tier, sum_usd;")
        .bind(token_address)
        .fetch_all(pool).await {
        Ok(result) => {
            let mut cheapest_and_most_expensive_trades_by_tier: BTreeMap<i32, Vec<SingleTrade>> = BTreeMap::new();
            for trade in result {
                let trades_map = cheapest_and_most_expensive_trades_by_tier.entry(trade.tier).or_insert(Vec::new());
                trades_map.push(trade.into());
            }
            cheapest_and_most_expensive_trades_by_tier
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            BTreeMap::new()
        }
    };
}

#[derive(FromRow)]
struct OrderDb {
    count: i64,
    status: String,
    buy_currency: String,
}

#[derive(FromRow)]
struct StatsDataTradesVolumeDb {
    total_trades: i64,
    total_in_buy_currency: Decimal,
    buy_currency: String,
    total_btc: Decimal,
    total_eth: Decimal,
    total_usd: Decimal,
    total_eur: Decimal,
    total_jpy: Decimal,
}
