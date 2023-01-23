use dotenvy::dotenv;
use mints_reader::read;

mod mints_reader;
mod model;
mod db_handler;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    dotenv().expect(".env file should be present");

    read();
}
