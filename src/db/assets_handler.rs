use crate::db::db_handler::Persistable;
use crate::model::asset::Asset;
use async_trait::async_trait;
use log::{error, info};
use sqlx::types::chrono::{DateTime, NaiveDateTime};
use sqlx::{query_as, Pool, Postgres, QueryBuilder};

pub struct AssetSaver;

#[async_trait]
impl Persistable<Asset> for AssetSaver {
    async fn create_one(&self, asset: &Asset, pool: &Pool<Postgres>) {
        let asset_result = &asset.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into asset (token_id, token_address, name, tier, solon, carbon, crypton,\
             silicon, hydrogen, hyperion, landmark, current_owner, created_on, updated_on) ",
        );

        query_builder.push_values(asset_result, |mut builder, res| {
            builder
                .push_bind(res.token_id.parse::<i32>().unwrap())
                .push_bind(&res.token_address)
                .push_bind(&res.metadata.name)
                .push_bind(res.metadata.tier)
                .push_bind(res.metadata.solon)
                .push_bind(res.metadata.carbon)
                .push_bind(res.metadata.crypton)
                .push_bind(res.metadata.silicon)
                .push_bind(res.metadata.hydrogen)
                .push_bind(res.metadata.hyperion)
                .push_bind(&res.metadata.landmark)
                .push_bind(&res.current_owner)
                .push_bind(DateTime::parse_from_rfc3339(&res.created_at).unwrap())
                .push_bind(DateTime::parse_from_rfc3339(&res.updated_at).unwrap());
        });

        let query = query_builder
            .push(" ON CONFLICT (token_id) DO UPDATE SET current_owner = EXCLUDED.current_owner, updated_on = EXCLUDED.updated_on;")
            .build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {}", e)
            }
        }
    }

    async fn get_last_timestamp(&self, pool: &Pool<Postgres>) -> Option<NaiveDateTime> {
        let result: (Option<NaiveDateTime>,) = query_as("select max(updated_on) from asset")
            .fetch_one(pool)
            .await
            .unwrap_or_else(|e| {
                error!("Couldn't fetch data! {}", e);
                (None,)
            });

        result.0
    }
}
