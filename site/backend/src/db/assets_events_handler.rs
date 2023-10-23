use crate::db::db_model::TransactionDataDb;
use log::error;
use model::model::transaction::EventData;
use sqlx::{query, query_as, Pool, Postgres, Row};

pub async fn get_events_for_token_address_and_token_id(
    pool: &Pool<Postgres>,
    token_address: &String,
    token_id: &i32,
    page: &i32,
    event: &String,
) -> Option<EventData> {
    let total: i64 = match query("select count(timestamp) from events_view where (token_address=$1 and token_id=$2) and ($3 = 'All' or event = $3)")
        .bind(token_address)
        .bind(token_id)
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
        // no need to display the image for asset, so have to select those as nulls
        "select transaction_id, wallet_from, wallet_to, event, timestamp, currency, price, usd_price,
         null as token_address, null as token_id, null as name, null as image_url from events_view
         where (token_address=$1 and token_id=$2) and ($3 = 'All' or event = $3)
         order by timestamp desc
         limit 50 offset $4")
        .bind(token_address)
        .bind(token_id)
        .bind(event)
        .bind((page - 1) * 50)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(
            EventData {
                total,
                transactions: result.into_iter().map(|t| t.into()).collect(),
            }
        ),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}
