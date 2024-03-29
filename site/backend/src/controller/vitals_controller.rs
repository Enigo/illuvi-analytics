use crate::db::vitals_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/api/stat/vitals")]
pub async fn get_vitals(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match vitals_handler::get_all_vitals_for_token_address(&pool, &params.token_address)
        .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
