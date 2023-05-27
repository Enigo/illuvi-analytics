use crate::db::stats_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/stat/stats")]
pub async fn get_stats(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    return match stats_handler::get_all_stats_for_token_address(&pool, &params.token_address).await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
