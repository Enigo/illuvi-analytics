use crate::db::db_handler;
use crate::model::immutablex::collection::Collection;
use log::{error, info};
use sqlx::types::chrono::DateTime;
use sqlx::{Postgres, QueryBuilder};

pub async fn create_one(collection: &Collection) {
    let pool = db_handler::open_connection().await;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into collection (address, name, description, icon_url, collection_image_url,\
                                     project_id, project_owner_address, metadata_api_url, created_on, updated_on) ",
    );

    let collection_result = &collection.result;
    query_builder.push_values(collection_result, |mut builder, res| {
        builder
            .push_bind(&res.address)
            .push_bind(&res.name)
            .push_bind(&res.description)
            .push_bind(&res.icon_url)
            .push_bind(&res.collection_image_url)
            .push_bind(&res.project_id)
            .push_bind(&res.project_owner_address)
            .push_bind(&res.metadata_api_url)
            .push_bind(DateTime::parse_from_rfc3339(&res.created_at).unwrap())
            .push_bind(DateTime::parse_from_rfc3339(&res.updated_at).unwrap());
    });

    let query = query_builder
        .push(" ON CONFLICT (address) DO UPDATE SET updated_on = EXCLUDED.updated_on;")
        .build();
    match query.execute(&pool).await {
        Ok(result) => {
            info!("Inserted {} rows", result.rows_affected())
        }
        Err(e) => {
            error!("Couldn't insert values due to {e}")
        }
    }

    db_handler::close_connection(pool).await;
}