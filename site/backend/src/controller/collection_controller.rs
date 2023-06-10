use crate::db::collection_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/api/collection/collections")]
pub async fn get_collections(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<impl Responder> {
    return match collection_handler::get_all_collections(&pool).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}

#[get("/api/collection/collection")]
pub async fn get_collection(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match collection_handler::get_collection_for_address(&pool, &params.token_address).await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
