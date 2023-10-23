use crate::db::{wallet_events_handler, wallet_handler};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct WalletParams {
    wallet: String,
}

#[derive(Deserialize)]
pub struct EventsParams {
    wallet: String,
    page: i32,
    event: String,
}

#[get("/api/wallet/wallet")]
pub async fn get_wallet(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<WalletParams>,
) -> actix_web::Result<impl Responder> {
    return match wallet_handler::get_wallet(&pool, &params.wallet).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}

#[get("/api/wallet/events")]
pub async fn get_wallet_events(
    pool: web::Data<Pool<Postgres>>,
    params: web::Query<EventsParams>,
) -> actix_web::Result<impl Responder> {
    return match wallet_events_handler::get_wallet_events(
        &pool,
        &params.wallet,
        params.page,
        &params.event,
    )
    .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}
