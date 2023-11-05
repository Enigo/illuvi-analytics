use crate::api_reader::api_utils;
use crate::db::immutablex::mints_handler;
use crate::model::etherscan::{token, transaction};
use crate::utils::env_utils;
use ethabi::{decode, ParamType};
use futures::StreamExt;
use log::{error, info};
use rust_decimal::prelude::*;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::num::ParseIntError;

const ILLUVIUM_TEAM_WALLET: &str = "0xc57651549ba961b929fe7c589e22ae75e075008d";
const LAND_CONTRACT: &str = "0x7a47f7707c4b2f2b1def04a47cd8681d48eadeb8";
const LAND_CONTRACT_CREATION_BLOCK: &str = "14846665";
const LAND_FUNCTION_NAME: &str = "buyL2";

pub async fn read_land_transactions(pool: &Pool<Postgres>) {
    if !env_utils::as_parsed::<bool>("ETHERSCAN_ENABLED") {
        return;
    }
    let api_key = env_utils::as_string("ETHERSCAN_API_KEY");
    match mints_handler::fetch_all_lands_with_no_price_or_currency(&pool).await {
        Some(wallets) => {
            info!("Fetching price and currency for {} wallets", wallets.len());
            let mut futures = futures::stream::iter(wallets)
                .map(|wallet| process_wallet(wallet, &api_key, &pool))
                .buffer_unordered(3);

            // waiting for all to complete
            while let Some(_) = futures.next().await {}
        }
        _ => {}
    }
}

async fn process_wallet(wallet: String, api_key: &String, pool: &Pool<Postgres>) {
    if ILLUVIUM_TEAM_WALLET == wallet {
        // illuvium owned lands were minted for free on IMX directly
        // so it is quite complicated to to find a corresponding transaction on L1 and makes no sense as well
        mints_handler::update_price_and_currency_for_wallet(
            wallet.as_str(),
            0.0,
            String::from("(minted by Illuvium team)"),
            pool,
        )
        .await;
        return;
    }

    let mut page = 1;
    let mut transaction_to_token_id = HashMap::new();
    loop {
        let transactions = fetch_transactions(wallet.clone(), api_key, page).await;
        if transactions.is_empty() {
            break;
        }

        for res in transactions {
            if res.is_error == "1"
                || res.to != LAND_CONTRACT
                || !res.function_name.contains(LAND_FUNCTION_NAME)
            {
                continue;
            }

            let input_to_decode = res.input.replace(res.method_id.as_str(), "");
            match decode_input_and_get_token_id(input_to_decode.as_str()) {
                Ok(token_id) => {
                    if res.value == "0" {
                        transaction_to_token_id.insert(res.hash.clone(), token_id);
                    } else {
                        process_transaction(&wallet, &pool, res, token_id).await;
                    }
                }
                Err(e) => {
                    error!("Error decoding input {e}")
                }
            };
        }

        page += 1;
    }

    if !transaction_to_token_id.is_empty() {
        process_tokens(&wallet, transaction_to_token_id, api_key, pool).await;
    }
}

async fn fetch_transactions(
    wallet: String,
    api_key: &String,
    page: i8,
) -> Vec<transaction::TheResult> {
    let endpoint = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&page={}&offset=10000&startblock={}&endblock=99999999&sort=asc&apikey={}",
                           wallet, page, LAND_CONTRACT_CREATION_BLOCK, api_key);
    return match api_utils::fetch_single_api_response::<transaction::Transaction>(endpoint.as_str())
        .await
    {
        Some(transaction) => {
            if transaction.status == "1" {
                return transaction.result.unwrap();
            }
            return vec![];
        }
        _ => vec![],
    };
}

async fn process_tokens(
    wallet: &String,
    transaction_to_token_id: HashMap<String, i32>,
    api_key: &String,
    pool: &Pool<Postgres>,
) {
    fn convert_into_value(value_str: &str, token_decimal_str: &str) -> Decimal {
        let value = Decimal::from_str(value_str).unwrap();
        let token_decimal = Decimal::from_str(&format!(
            "1{}",
            "0".repeat(
                token_decimal_str
                    .parse::<u32>()
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
        ))
        .unwrap();
        return  value / token_decimal;
    }

    let mut page = 1;
    loop {
        let tokens = fetch_tokens(wallet.clone(), api_key, page).await;
        if tokens.is_empty() {
            break;
        }

        for res in tokens {
            if let Some(token_id) = transaction_to_token_id.get(&res.hash) {
                mints_handler::update_price_and_currency_for_wallet_and_token_id(
                    wallet.as_str(),
                    convert_into_value(res.value.as_str(), res.token_decimal.as_str()),
                    res.token_symbol.to_uppercase(),
                    token_id,
                    &pool,
                )
                .await;
            }
        }

        page += 1;
    }
}

async fn process_transaction(
    wallet: &String,
    pool: &&Pool<Postgres>,
    res: transaction::TheResult,
    token_id: i32,
) {
    // The value returned by the Etherscan API endpoint is in Wei, which is the smallest unit of ether
    let wei_value = Decimal::from_str(res.value.as_str()).unwrap();
    let ether_value = wei_value / Decimal::new(10i64.pow(18), 0);

    mints_handler::update_price_and_currency_for_wallet_and_token_id(
        wallet.as_str(),
        ether_value,
        String::from("ETH"),
        &token_id,
        &pool,
    )
    .await;
}

async fn fetch_tokens(wallet: String, api_key: &String, page: i8) -> Vec<token::TheResult> {
    let endpoint = format!("https://api.etherscan.io/api?module=account&action=tokentx&address={}&page={}&offset=10000&startblock={}&endblock=99999999&sort=asc&apikey={}",
                           wallet, page, LAND_CONTRACT_CREATION_BLOCK, api_key);
    return match api_utils::fetch_single_api_response::<token::Token>(endpoint.as_str()).await {
        Some(token) => {
            if token.status == "1" {
                return token.result.unwrap();
            }
            return vec![];
        }
        _ => vec![],
    };
}

fn decode_input_and_get_token_id(input_to_decode: &str) -> Result<i32, Box<dyn std::error::Error>> {
    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    let input_decoded = decode_hex(input_to_decode)?;
    let input_params = vec![
        ParamType::Tuple(vec![
            ParamType::Uint(32),
            ParamType::Uint(32),
            ParamType::Uint(8),
            ParamType::Uint(16),
            ParamType::Uint(16),
            ParamType::Uint(8),
            ParamType::Uint(16),
        ]),
        ParamType::FixedBytes(32),
    ];
    let decoded_input = decode(&input_params, &input_decoded[..])?;
    let token_id = decoded_input[0]
        .clone()
        .into_tuple()
        .ok_or("Error: could not convert decoded input into tuple")?[0]
        .clone()
        .into_uint()
        .ok_or("Error: could not convert tuple element into uint")?
        .as_u32();
    Ok(token_id as i32)
}
