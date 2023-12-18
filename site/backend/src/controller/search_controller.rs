use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::db::assets_handler;

#[derive(Deserialize)]
pub struct SearchParams {
    search: String,
}

#[get("/api/search")]
pub async fn get_search_results(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<SearchParams>,
) -> actix_web::Result<impl Responder> {
    return match assets_handler::get_search_results(&pool, &params.search).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}
