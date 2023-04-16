use crate::db::db_handler;
use log::error;
use model::model::mint::MintData;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query_as, FromRow};

pub async fn get_all_mints_for_token_address(token_address: &String) -> Option<Vec<MintData>> {
    let pool = db_handler::open_connection().await;

    let mint: Option<Vec<MintDb>> = match query_as(
        "select m.transaction_id, m.wallet, m.token_id, m.token_address, m.minted_on, a.name from mint m \
        join asset a on m.token_id = a.token_id where m.token_address=$1")
        .bind(token_address)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    let result = match mint {
        Some(res) => Some(res.into_iter().map(|m| m.into()).collect()),
        None => None,
    };

    db_handler::close_connection(pool).await;

    return result;
}

// https://github.com/launchbadge/sqlx/discussions/1886
// sqlx is not wasm compatible, so the dependency cannot be used in the `ui` module
#[derive(FromRow)]
struct MintDb {
    transaction_id: i32,
    wallet: String,
    token_id: i32,
    token_address: String,
    minted_on: NaiveDateTime,
    name: String,
}

impl From<MintDb> for MintData {
    fn from(mint: MintDb) -> Self {
        Self {
            transaction_id: mint.transaction_id,
            wallet: mint.wallet,
            token_id: mint.token_id,
            token_address: mint.token_address,
            minted_on: mint.minted_on,
            name: mint.name,
        }
    }
}
