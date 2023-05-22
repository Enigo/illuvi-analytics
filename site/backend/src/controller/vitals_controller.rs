use crate::db::vitals_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
}

#[get("/stat/vitals")]
pub async fn get_vitals(params: web::Query<Params>) -> actix_web::Result<impl Responder> {
    return match vitals_handler::get_all_vitals_for_token_address(&params.token_address).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(result) => Ok(HttpResponse::Ok().json(result)),
    };
}
