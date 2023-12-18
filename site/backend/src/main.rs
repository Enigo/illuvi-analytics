use crate::controller::{
    assets_controller::get_asset, assets_controller::get_events,
    collection_controller::get_collection, collection_controller::get_collections,
    mints_controller::get_mints, search_controller::get_search_results,
    stats_controller::get_stats, vitals_controller::get_vitals, wallet_controller::get_wallet,
    wallet_controller::get_wallet_events,
};
use crate::db::db_handler;
use crate::utils::env_utils;
use actix_cors::Cors;
use actix_web::middleware::Compress;
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
            .service(get_events)
            .service(get_collections)
            .service(get_collection)
            .service(get_stats)
            .service(get_search_results)
            .service(get_vitals)
            .service(get_wallet)
            .service(get_wallet)
            .service(get_wallet_events)
            .wrap(
                Cors::default()
                    .allowed_origin(&env_utils::as_string("ALLOWED_ORIGIN"))
                    .allowed_methods(vec!["GET"]),
            )
            .wrap(Compress::default())
    })
    .bind((
        env_utils::as_string("ENDPOINT"),
        env_utils::as_parsed::<u16>("PORT"),
    ))?
    .run()
    .await
}
