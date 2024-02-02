use crate::api_reader::api_utils;
use crate::db::immutablex::collection_handler;
use crate::model::immutablex::collection::Collection;
use crate::utils::env_utils;
use sqlx::{Pool, Postgres};

const COLLECTION_URL: &str = "https://api.x.immutable.com/v1/collections";

pub async fn read_collections(pool: &Pool<Postgres>) -> Vec<String> {
    if env_utils::as_parsed::<bool>("COLLECTIONS_ENABLED") {
        // there are some "scam" projects that match Illuvium keywords, so there is no other way to fetch the collections
        let collections = vec![
            "0x07fb805d026194d188014fc7303e69f412eb7cb1",
            "0xc1f1da534e227489d617cd742481fd5a23f6a003",
            "0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24",
            "0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8",
            "0x9e0d99b864e1ac12565125c5a82b59adea5a09cd",
            "0x0d78b8aeddb8d3c8b8903a474f8a91855bfdf6f2",
        ];

        for collection in collections {
            let result = api_utils::fetch_single_api_response::<Collection>(
                format!("{}/{}", COLLECTION_URL, collection).as_str(),
                &vec![],
            )
            .await;
            if result.is_some() {
                collection_handler::create_one(&result.unwrap(), &pool).await;
            }
        }
    }

    collection_handler::fetch_all_collections(&pool).await
}
