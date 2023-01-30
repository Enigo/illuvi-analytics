use crate::model::asset::Asset;
use log::{error, info};
use sqlx::{query, query_as, FromRow, Pool, Postgres};
use std::collections::HashSet;

pub async fn save_asset(asset: &Asset, pool: &Pool<Postgres>) {
    match query("insert into asset (token_id, token_address, name, tier, solon, carbon, crypton, silicon, hydrogen, hyperion, landmark)\
                 values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
        .bind(&asset.token_id.parse::<i32>().unwrap())
        .bind(&asset.token_address)
        .bind(&asset.metadata.name)
        .bind(&asset.metadata.tier)
        .bind(&asset.metadata.solon)
        .bind(&asset.metadata.carbon)
        .bind(&asset.metadata.crypton)
        .bind(&asset.metadata.silicon)
        .bind(&asset.metadata.hydrogen)
        .bind(&asset.metadata.hyperion)
        .bind(&asset.metadata.landmark)
        .execute(pool).await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {}", e)
        }
    }
}

pub async fn get_all_token_ids(connection: &Pool<Postgres>) -> Option<HashSet<i32>> {
    match query_as::<Postgres, AssetDb>("select token_id from asset")
        .fetch_all(connection)
        .await
    {
        Ok(result) => Some(result.iter().map(|mint| mint.token_id).collect()),
        Err(e) => {
            error!("Error fetching data {}", e);
            None
        }
    }
}

#[derive(FromRow)]
struct AssetDb {
    token_id: i32,
}
