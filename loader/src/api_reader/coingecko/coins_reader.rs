use crate::api_reader::api_utils;
use crate::db::coingecko::{coins_handler, coins_history_handler};
use crate::model::coingecko::coin::Coin;
use crate::model::coingecko::coin_history::CoinHistory;
use crate::utils::env_utils;
use futures::StreamExt;
use log::{info, warn};
use sqlx::types::chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use std::time::Duration;

const LIST_ENDPOINT: &str = "https://api.coingecko.com/api/v3/coins/list";
const HISTORY_ENDPOINT: &str =
    "https://api.coingecko.com/api/v3/coins/{}/history?localization=false&date={}";

pub async fn read_coins(pool: &Pool<Postgres>) {
    if env_utils::as_parsed::<bool>("COINS_ENABLED") {
        fetch_coins(&pool).await;
        fetch_coins_history(&pool).await;
    }
}

async fn fetch_coins(pool: &Pool<Postgres>) {
    match coins_handler::get_all_missing_symbols_for_filled_or_active_orders(&pool).await {
        Some(missing_symbols) => {
            if !missing_symbols.is_empty() {
                info!("Fetching ids for {:?}", missing_symbols);
                match api_utils::fetch_single_api_response::<Vec<Coin>>(LIST_ENDPOINT, &vec![])
                    .await
                {
                    Some(coins) => {
                        info!(
                            "Processing response from {LIST_ENDPOINT} with {} coins",
                            coins.len()
                        );
                        // there are 2 ETH and 3 USDC, only one of those is needed
                        let excluded_ids = vec![
                            "force-bridge-usdc",
                            "usd-coin-avalanche-bridged-usdc-e",
                            "ethereum-wormhole",
                            "apemove",
                        ];
                        for coin in coins {
                            if missing_symbols.contains(&coin.symbol.to_uppercase())
                                && !excluded_ids.contains(&coin.id.as_str())
                            {
                                coins_handler::create_one(&coin, &pool).await;
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        _ => {}
    }
}

// Returned data is at 00:00:00 UTC for the given day
async fn fetch_coins_history(pool: &Pool<Postgres>) {
    async fn process_date(date: &NaiveDate, symbol_id: &String, pool: &Pool<Postgres>) {
        // format macro cannot be used with consts
        let url = HISTORY_ENDPOINT.replacen("{}", symbol_id, 1).replacen(
            "{}",
            date.format("%d-%m-%Y").to_string().as_str(),
            1,
        );
        match api_utils::fetch_single_api_response::<CoinHistory>(url.as_str(), &vec![]).await {
            Some(coin_history) => {
                info!("Processing response from {url}");
                if coin_history.market_data.is_some() {
                    coins_history_handler::create_one(coin_history, date, pool).await;
                } else {
                    warn!(
                        "No data for id {} and symbol {} and date {}",
                        coin_history.id, coin_history.symbol, date
                    );
                }
            }
            None => {}
        }
    }
    let missing_pairs =
        coins_history_handler::get_all_missing_distinct_date_to_id_pairs(&pool).await;
    if !missing_pairs.is_empty() {
        info!(
            "Processing historical data for {:?}, total of {}",
            missing_pairs,
            missing_pairs.len()
        );
        for chunk in missing_pairs.chunks(5) {
            futures::stream::iter(chunk)
                .for_each(|pair| process_date(&pair.0, &pair.1, pool))
                .await;
            if missing_pairs.len() > 5 {
                // Limitation is 10-30 calls/MINUTE
                let sleep_for = 65;
                info!("Sleeping for {sleep_for} seconds to avoid rate limit!");
                tokio::time::sleep(Duration::from_secs(sleep_for)).await;
            }
        }
    }
}
