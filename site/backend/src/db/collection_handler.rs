use crate::db::db_handler;
use log::error;
use model::model::collection::CollectionData;
use sqlx::{query_as, FromRow};

pub async fn get_all_collections() -> Option<Vec<CollectionData>> {
    let pool = db_handler::open_connection().await;

    let query_result: Option<Vec<CollectionDataDb>> = match query_as(
        "select address, name, collection_image_url from collection order by created_on",
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

#[derive(FromRow)]
struct CollectionDataDb {
    pub address: String,
    pub name: String,
    pub collection_image_url: String,
}

impl From<CollectionDataDb> for CollectionData {
    fn from(transfer_data: CollectionDataDb) -> Self {
        Self {
            address: transfer_data.address,
            name: transfer_data.name,
            collection_image_url: transfer_data.collection_image_url,
        }
    }
}
