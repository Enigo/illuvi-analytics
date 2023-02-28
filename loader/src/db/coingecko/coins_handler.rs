use crate::model::coingecko::coin::Coin;
use log::{error, info};
use sqlx::{query, query_scalar, Pool, Postgres};

pub async fn create_one(coin: &Coin, pool: &Pool<Postgres>) {
    match query("insert into coin (id, symbol, name) values ($1, $2, $3)")
        .bind(&coin.id)
        .bind(&coin.symbol.to_uppercase())
        .bind(&coin.name)
        .execute(pool)
        .await
    {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {e}")
        }
    }
}

pub async fn get_all_missing_symbols_for_filled_orders(
    pool: &Pool<Postgres>,
) -> Option<Vec<String>> {
    return match query_scalar(
        "select distinct(od.buy_currency)
                               from order_data od
                               where not exists(
                                       select
                                       from coin c
                                       where c.symbol = od.buy_currency)
                                 and od.status = 'filled';",
    )
    .fetch_all(pool)
    .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}
