use crate::db::mints_handler;
use actix_web::{get, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/mint/mints")]
pub async fn get_mints(params: web::Query<Params>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        mints_handler::get_all_mints_for_token_address(&params.token_address).await,
    ))
}

#[get("/mint/token_addresses")]
pub async fn get_all_token_addresses() -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        mints_handler::get_distinct_token_addresses().await,
    ))
}
