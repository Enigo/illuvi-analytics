use crate::controller::{
    assets_controller::get_asset, collection_controller::get_collection,
    collection_controller::get_collections, mints_controller::get_mints,
    stats_controller::get_stats, vitals_controller::get_vitals,
};
use crate::db::db_handler;
use actix_cors::Cors;
use actix_web::web::Data;
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

    let pool = db_handler::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_mints)
            .service(get_asset)
            .service(get_collections)
            .service(get_collection)
            .service(get_stats)
            .service(get_vitals)
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET"]),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
