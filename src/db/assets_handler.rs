use crate::db::db_handler::Persistable;
use crate::model::asset::Asset;
use async_trait::async_trait;
use log::{error, info};
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct AssetSaver;

#[async_trait]
impl Persistable<Asset> for AssetSaver {
    async fn persist_one(&self, asset: &Asset, pool: &Pool<Postgres>) {
        let asset_result = &asset.result;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into asset (token_id, token_address, name, tier, solon, carbon, crypton, silicon, hydrogen, hyperion, landmark) ",
        );

        query_builder.push_values(asset_result, |mut builder, res| {
            builder
                .push_bind(res.token_id.parse::<i32>().unwrap())
                .push_bind(res.token_address.clone())
                .push_bind(res.metadata.name.clone())
                .push_bind(res.metadata.tier)
                .push_bind(res.metadata.solon)
                .push_bind(res.metadata.carbon)
                .push_bind(res.metadata.crypton)
                .push_bind(res.metadata.silicon)
                .push_bind(res.metadata.hydrogen)
                .push_bind(res.metadata.hyperion)
                .push_bind(res.metadata.landmark.clone());
        });

        let query = query_builder.build();
        match query.execute(pool).await {
            Ok(result) => {
                info!("Inserted {} rows", result.rows_affected())
            }
            Err(e) => {
                error!("Couldn't insert values due to {}", e)
            }
        }
    }
}
