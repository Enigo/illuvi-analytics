use crate::model::coingecko::coin_history::CoinHistory;
use log::{error, info};
use sqlx::types::chrono::NaiveDate;
use sqlx::{query, query_as, FromRow, Pool, Postgres};

pub async fn create_one(coin_history: CoinHistory, date: &NaiveDate, pool: &Pool<Postgres>) {
    let current_price = coin_history.market_data.unwrap().current_price;
    match query("insert into coin_history (symbol, btc, eth, eur, jpy, usd, datestamp) values ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (symbol, datestamp) DO UPDATE SET btc = EXCLUDED.btc, eth = EXCLUDED.eth, eur = EXCLUDED.eur, jpy = EXCLUDED.jpy, usd = EXCLUDED.usd;
    ")
        .bind(coin_history.symbol.to_uppercase())
        .bind(current_price.btc)
        .bind(current_price.eth)
        .bind(current_price.eur)
        .bind(current_price.jpy)
        .bind(current_price.usd)
        .bind(date)
        .execute(pool).await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {e}")
        }
    }
}

pub async fn get_all_missing_distinct_date_to_id_pairs(
    pool: &Pool<Postgres>,
) -> Vec<(NaiveDate, String)> {
    return match query_as::<_, CoinHistoryData>("select distinct(dates) as datestamp, id from (
                                                                select distinct m.minted_on::date as dates, c.id
                                                                from mint m
                                                                    join coin c on c.symbol = m.currency
                                                                where not exists (
                                                                   select 1
                                                                   from coin_history ch
                                                                   where ch.datestamp = m.minted_on::date
                                                                     and ch.symbol = m.currency
                                                                )
                                                                union
                                                                select distinct od.updated_on::date as dates, c.id
                                                                from order_data od
                                                                    join coin c on c.symbol = od.buy_currency
                                                                where not exists (
                                                                   select 1
                                                                   from coin_history ch
                                                                   where ch.datestamp = od.updated_on::date
                                                                     and ch.symbol = od.buy_currency
                                                                )
                                                                and od.status = 'filled'
                                                                union
                                                                select now()::date as dates, c.id
                                                                from order_data od
                                                                         join coin c on c.symbol = od.buy_currency
                                                                where not exists (
                                                                        select 1
                                                                        from coin_history ch
                                                                        where ch.datestamp = now()::date
                                                                          and ch.symbol = od.buy_currency
                                                                    )
                                                                  and od.status = 'active'
                                                            ) res;")
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.into_iter()
            .map(|data| (data.datestamp, data.id))
            .collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

#[derive(FromRow)]
struct CoinHistoryData {
    datestamp: NaiveDate,
    id: String,
}
