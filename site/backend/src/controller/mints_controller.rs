use crate::db::mints_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
    page: i16,
}

#[get("/mint/mints")]
pub async fn get_mints(params: web::Query<Params>) -> actix_web::Result<impl Responder> {
    return match mints_handler::get_all_mints_for_token_address(&params.token_address, params.page)
        .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
