use crate::db::mints_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
    page: i32,
}

#[get("/api/mint/mints")]
pub async fn get_mints(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match mints_handler::get_all_mints_for_token_address(
        &pool,
        &params.token_address,
        params.page,
    )
    .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
