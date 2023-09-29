use crate::db::{assets_events_handler, assets_handler};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
    token_id: i32,
}

#[get("/api/asset/asset")]
pub async fn get_asset(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match assets_handler::get_asset_for_token_address_and_token_id(
        &pool,
        &params.token_address,
        &params.token_id,
    )
    .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}

#[get("/api/asset/events")]
pub async fn get_events(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match assets_events_handler::get_events_for_token_address_and_token_id(
        &pool,
        &params.token_address,
        &params.token_id,
    )
    .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}
