use crate::db::db_handler;
use log::error;
use model::model::mint::{Mint, MintData};
use sqlx::{query, query_as, FromRow, Row};

pub async fn get_all_mints_for_token_address(
    token_address: &String,
    page: i16,
) -> Option<MintData> {
    let pool = db_handler::open_connection().await;

    let total: i64 = match query("select count(token_id) from mint where token_address=$1")
        .bind(token_address)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => result.get(0),
        Err(e) => {
            error!("Error fetching data: {e}");
            0
        }
    };

    let result: Option<MintData> = match query_as::<_, MintDb>(
        "select m.token_id, m.token_address, a.name from mint m
        join asset a on m.token_id = a.token_id where m.token_address=$1
        order by a.tier desc, m.token_id
        limit 50
        offset $2",
    )
    .bind(token_address)
    .bind((page - 1) * 50)
    .fetch_all(&pool)
    .await
    {
        Ok(res) => {
            let mints = res.into_iter().map(|mint| mint.into()).collect();
            Some(MintData { total, mints })
        }
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    db_handler::close_connection(pool).await;

    return result;
}

// https://github.com/launchbadge/sqlx/discussions/1886
// sqlx is not wasm compatible, so the dependency cannot be used in the `ui` module
#[derive(FromRow)]
struct MintDb {
    token_id: i32,
    token_address: String,
    name: String,
}

impl From<MintDb> for Mint {
    fn from(mint: MintDb) -> Self {
        Self {
            token_id: mint.token_id,
            token_address: mint.token_address,
            name: mint.name,
        }
    }
}
