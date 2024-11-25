use crate::db::db_model::TransactionDataDb;
use log::error;
use model::model::transaction::EventData;
use sqlx::{query, query_as, Pool, Postgres, Row};

pub async fn get_wallet_events(
    pool: &Pool<Postgres>,
    wallet: &String,
    page: i32,
    event: &String,
) -> Option<EventData> {
    let total: i64 = match query("select count(*) from events_view where (wallet_from=$1 or wallet_to=$1) and ($2 = 'All' or event = $2)")
        .bind(wallet)
        .bind(event)
        .fetch_one(pool)
        .await
    {
        Ok(result) => result.get(0),
        Err(e) => {
            error!("Error fetching data: {e}");
            0
        }
    };

    return match query_as::<_, TransactionDataDb>(
        "select emv.transaction_id, emv.wallet_from, emv.wallet_to, emv.event, emv.timestamp, emv.currency, emv.price, emv.usd_price,
            emv.token_address, emv.token_id, a.metadata->>'name' as name, a.metadata->>'image_url' as image_url from events_view emv
         join asset a on a.token_address = emv.token_address and a.token_id = emv.token_id
         where (emv.wallet_from=$1 or emv.wallet_to=$1) and ($2 = 'All' or emv.event = $2)
         order by emv.timestamp desc, emv.token_id
         limit 50 offset $3")
        .bind(wallet)
        .bind(event)
        .bind((page - 1) * 50)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(
            EventData {
                total,
                transactions: result.into_iter().map( | t| t.into()).collect()
            }
        ),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}
