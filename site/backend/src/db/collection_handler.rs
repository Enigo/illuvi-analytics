use log::error;
use model::model::collection::CollectionData;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query_as, FromRow, Pool, Postgres};

pub async fn get_all_collections(pool: &Pool<Postgres>) -> Option<Vec<CollectionData>> {
    return match query_as::<_, CollectionDataDb>(
        "select address, name, collection_image_url, description, created_on from collection order by created_on",
    )
        .fetch_all(pool)
        .await
    {
        Ok(result) => Some(result.into_iter().map(|t| t.into()).collect()),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
    };
}

pub async fn get_collection_for_address(
    pool: &Pool<Postgres>,
    address: &String,
) -> Option<CollectionData> {
    return match query_as::<_, CollectionDataDb>(
        "select address, name, collection_image_url, description, created_on from collection where address=$1",
    )
        .bind(address)
        .fetch_one(pool)
        .await
    {
        Ok(result) => Some(result.into()),
        Err(e) => {
            error!("Error fetching data: {e}");
            None
        }
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
