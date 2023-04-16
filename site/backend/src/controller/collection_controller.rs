use crate::db::collection_handler;
use actix_web::{get, HttpResponse, Responder};

#[get("/collection/collections")]
pub async fn get_collections() -> actix_web::Result<impl Responder> {
    return match collection_handler::get_all_collections().await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
