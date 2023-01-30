use api_reader::reader::read;
use dotenvy::dotenv;

mod api_reader;
mod db;
mod env_utils;
mod model;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    dotenv().expect(".env file should be present");

    read();
}
