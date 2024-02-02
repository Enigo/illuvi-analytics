use crate::model::immutablex::collection::Collection;
use log::{error, info};
use sqlx::types::chrono::DateTime;
use sqlx::{query, query_scalar, Pool, Postgres};

pub async fn create_one(collection: &Collection, pool: &Pool<Postgres>) {
    match query("insert into collection (address, name, description, icon_url, collection_image_url,
                                     project_id, project_owner_address, metadata_api_url, created_on, updated_on)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ON CONFLICT (address) DO UPDATE SET updated_on = EXCLUDED.updated_on;")
        .bind(&collection.address)
        .bind(&collection.name)
        .bind(&collection.description)
        .bind(&collection.icon_url)
        .bind(&collection.collection_image_url)
        .bind(&collection.project_id)
        .bind(&collection.project_owner_address)
        .bind(&collection.metadata_api_url)
        .bind(DateTime::parse_from_rfc3339(&collection.created_at).unwrap())
        .bind(DateTime::parse_from_rfc3339(&collection.updated_at).unwrap())
        .execute(pool).await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {e}")
        }
    }
}

pub async fn fetch_all_collections(pool: &Pool<Postgres>) -> Vec<String> {
    return query_scalar("select address from collection")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|e| {
            error!("Error fetching data: {e}");
            vec![]
        });
}
