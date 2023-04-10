use crate::controller::assets_controller::get_asset;
use crate::controller::mints_controller::{get_all_token_addresses, get_mints};
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod controller;
mod db;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    dotenv().expect(".env file should be present");

    HttpServer::new(|| {
        App::new()
            .service(get_mints)
            .service(get_all_token_addresses)
            .service(get_asset)
            // cors need to be configured correctly
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
