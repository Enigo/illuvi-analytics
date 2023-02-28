use crate::api_reader::api_utils;
use crate::db::coingecko::{coins_handler, coins_history_handler};
use crate::db::db_handler;
use crate::model::coingecko::coin::Coin;
use crate::model::coingecko::coin_history::CoinHistory;
use crate::utils::env_utils;
use futures::StreamExt;
use log::{error, info};
use sqlx::types::chrono::{NaiveDate, Utc};
use sqlx::{Pool, Postgres};
use std::thread;
use std::time::Duration;

const LIST_ENDPOINT: &str = "https://api.coingecko.com/api/v3/coins/list";
const HISTORY_ENDPOINT: &str =
    "https://api.coingecko.com/api/v3/coins/{}/history?localization=false&date={}";

pub async fn read_coins() {
    if env_utils::as_parsed::<bool>("COINS_ENABLED") {
        let pool = db_handler::open_connection().await;

        fetch_coins(&pool).await;
        fetch_coins_history(&pool).await;

        db_handler::close_connection(pool).await;
    }
}

async fn fetch_coins(pool: &Pool<Postgres>) {
    match coins_handler::get_all_missing_symbols_for_filled_orders(&pool).await {
        Some(missing_symbols) => {
            if !missing_symbols.is_empty() {
                info!("Fetching ids for {:?}", missing_symbols);
                match api_utils::fetch_single_api_response::<Vec<Coin>>(LIST_ENDPOINT).await {
                    Ok(coins) => {
                        info!(
                            "Processing response from {LIST_ENDPOINT} with {} coins",
                            coins.len()
                        );
                        // there are 2 ETH and 3 USDC, only one of those is needed
                        let excluded_ids = vec![
                            "force-bridge-usdc",
                            "usd-coin-avalanche-bridged-usdc-e",
                            "ethereum-wormhole",
                        ];
                        for coin in coins {
                            if missing_symbols.contains(&coin.symbol.to_uppercase())
                                && !excluded_ids.contains(&coin.id.as_str())
                            {
                                coins_handler::create_one(&coin, &pool).await;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Couldn't read {LIST_ENDPOINT}! {e}");
                    }
                }
            }
        }
        _ => {}
    }
}

async fn fetch_coins_history(pool: &Pool<Postgres>) {
    async fn process_date(date: &NaiveDate, symbol_id: &String, pool: &Pool<Postgres>) {
        // format macro cannot be used with consts
        let url = HISTORY_ENDPOINT.replacen("{}", symbol_id, 1)
            .replacen("{}", date.format("%d-%m-%Y").to_string().as_str(), 1);
        match api_utils::fetch_single_api_response::<CoinHistory>(url.as_str()).await {
            Ok(coin_history) => {
                info!("Processing response from {url}");
                coins_history_handler::create_one(&coin_history, date, pool).await;
            }
            Err(e) => {
                error!("Couldn't read {url}! {e}");
            }
        }
    }

    match coins_history_handler::get_all_missing_distinct_date_to_id_pairs_for_filled_orders(&pool)
        .await
    {
        Some(pairs) => {
            let values_to_process = pairs
                .into_iter()
                // only want past dates
                .filter(|pair| pair.0 != Utc::now().date_naive())
                .collect::<Vec<_>>();
            if !values_to_process.is_empty() {
                info!(
                    "Processing historical data for {:?}, total of {}",
                    values_to_process,
                    values_to_process.len()
                );
                for chunk in values_to_process.chunks(8) {
                    futures::stream::iter(chunk)
                        .for_each(|pair| process_date(&pair.0, &pair.1, pool))
                        .await;
                    if values_to_process.len() > 8 {
                        // Limitation is 10-30 calls/MINUTE
                        let sleep_for = 65;
                        info!("Sleeping for {sleep_for} seconds to avoid rate limit!");
                        thread::sleep(Duration::from_secs(sleep_for));
                    }
                }
            }
        }
        _ => {}
    };
}
