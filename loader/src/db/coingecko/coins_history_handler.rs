use crate::model::coingecko::coin_history::CoinHistory;
use log::{error, info};
use sqlx::types::chrono::NaiveDate;
use sqlx::{query, query_as, FromRow, Pool, Postgres};

pub async fn create_one(coin_history: &CoinHistory, date: &NaiveDate, pool: &Pool<Postgres>) {
    let current_price = &coin_history.market_data.current_price;
    match query("insert into coin_history (symbol, btc, eth, eur, jpy, usd, datestamp) values ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&coin_history.symbol.to_uppercase())
        .bind(&current_price.btc)
        .bind(&current_price.eth)
        .bind(&current_price.eur)
        .bind(&current_price.jpy)
        .bind(&current_price.usd)
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

pub async fn get_all_missing_distinct_date_to_id_pairs_for_filled_orders(
    pool: &Pool<Postgres>,
) -> Option<Vec<(NaiveDate, String)>> {
    let result: Option<Vec<CoinHistoryData>> = match query_as("select distinct(od.updated_on::DATE) as datestamp, c.id
                                                                from order_data od
                                                                         join coin c on c.symbol = od.buy_currency
                                                                where not exists(
                                                                        select
                                                                        from coin_history ch
                                                                        where ch.datestamp = od.updated_on::date
                                                                          and ch.symbol = od.buy_currency)
                                                                  and od.status = 'filled';")
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
        Some(res) => Some(
            res.into_iter()
                .map(|data| (data.datestamp, data.id))
                .collect(),
        ),
        None => None,
    }
}

#[derive(FromRow)]
struct CoinHistoryData {
    datestamp: NaiveDate,
    id: String,
}
