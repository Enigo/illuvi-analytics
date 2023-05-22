use crate::db::collection_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/collection/collections")]
pub async fn get_collections() -> actix_web::Result<impl Responder> {
    return match collection_handler::get_all_collections().await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}

#[get("/collection/collection")]
pub async fn get_collection(params: web::Query<Params>) -> actix_web::Result<impl Responder> {
    return match collection_handler::get_collection_for_address(&params.token_address).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
