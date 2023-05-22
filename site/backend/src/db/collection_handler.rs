use crate::db::db_handler;
use log::error;
use model::model::collection::CollectionData;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query_as, FromRow};

pub async fn get_all_collections() -> Option<Vec<CollectionData>> {
    let pool = db_handler::open_connection().await;

    let query_result: Option<Vec<CollectionDataDb>> = match query_as(
        "select address, name, collection_image_url, description, created_on from collection order by created_on",
    )
        .fetch_all(&pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    let result = match query_result {
        Some(res) => Some(res.into_iter().map(|t| t.into()).collect()),
        None => None,
    };

    db_handler::close_connection(pool).await;

    return result;
}

pub async fn get_collection_for_address(address: &String) -> Option<CollectionData> {
    let pool = db_handler::open_connection().await;

    let result: Option<CollectionDataDb> = match query_as(
        "select address, name, collection_image_url, description, created_on from collection where address=$1",
    )
        .bind(address)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };

    db_handler::close_connection(pool).await;

    return match result {
        Some(res) => Some(res.into()),
        None => None,
    };
}

#[derive(FromRow)]
struct CollectionDataDb {
    pub address: String,
    pub name: String,
    pub collection_image_url: String,
    pub description: String,
    pub created_on: NaiveDateTime,
}

impl From<CollectionDataDb> for CollectionData {
    fn from(data: CollectionDataDb) -> Self {
        Self {
            address: data.address,
            name: data.name,
            collection_image_url: data.collection_image_url,
            description: data.description,
            created_on: data.created_on,
        }
    }
}
