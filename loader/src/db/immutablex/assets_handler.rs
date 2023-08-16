use crate::db::immutablex::persistable::Persistable;
use crate::model::immutablex::asset::Asset;
use async_trait::async_trait;
use log::{error, info, warn};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::types::Json;
use sqlx::{query, query_as, Pool, Postgres, QueryBuilder, Row};
use std::collections::HashMap;

pub struct AssetSaver;

#[async_trait]
impl Persistable<Asset> for AssetSaver {
    async fn create_one(&self, asset: &Asset, pool: &Pool<Postgres>) {
        let asset_result = &asset.result;
        // attribute column is set by Postgres function
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into asset (token_id, token_address, metadata, current_owner, created_on, updated_on) ",
        );

        query_builder.push_values(asset_result, |mut builder, res| {
            let metadata = &res.metadata;
            let mut metadata_values = HashMap::new();
            if metadata.is_none() {
                warn!("Missing metadata for {}", res.token_id)
            } else {
                metadata_values = metadata.clone().unwrap();
            }
            builder
                .push_bind(res.token_id.parse::<i32>().unwrap())
                .push_bind(&res.token_address)
                .push_bind(Json(metadata_values))
                .push_bind(&res.current_owner)
                .push_bind(DateTime::parse_from_rfc3339(&res.created_at).unwrap())
                .push_bind(DateTime::parse_from_rfc3339(&res.updated_at).unwrap());
        });

        let query = query_builder
            .push(" ON CONFLICT (token_id, token_address) DO UPDATE SET current_owner = EXCLUDED.current_owner,
                        updated_on = EXCLUDED.updated_on, metadata = EXCLUDED.metadata;")
            .build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {e}")
            }
        }
    }

    async fn get_last_timestamp(
        &self,
        pool: &Pool<Postgres>,
        token_address: &String,
    ) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) =
            query_as("select max(updated_on) from asset where token_address=$1")
                .bind(token_address)
                .fetch_one(pool)
                .await
                .unwrap_or_else(|e| {
                    error!("Couldn't fetch data! {e}");
                    (None,)
                });

        result.0
    }
}

pub async fn fetch_all_assets_with_no_metadata(pool: &Pool<Postgres>) -> Vec<(String, i32)> {
    return match query("select token_address, token_id from asset where metadata='{}'")
        .fetch_all(pool)
        .await
    {
        Ok(result) => result.iter().map(|row| (row.get(0), row.get(1))).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

pub async fn update_metadata(
    metadata: HashMap<String, serde_json::Value>,
    token_address: &String,
    token_id: &i32,
    pool: &Pool<Postgres>,
) {
    match query("UPDATE asset SET metadata = $1 WHERE token_address=$2 and token_id=$3")
        .bind(Json(metadata))
        .bind(token_address)
        .bind(token_id)
        .execute(pool)
        .await
    {
        Ok(res) => {
            info!(
                "Updated {} assets metadata for {token_address} and {token_id}",
                res.rows_affected()
            )
        }
        Err(e) => {
            error!("Error updating assets metadata: {e}");
        }
    }
}
