use crate::Route;
use chrono::TimeZone;
use chrono::{Local, NaiveDateTime};
use model::model::price::Price;
use yew::*;
use yew_router::prelude::*;

const IMMUTASCAN_WALLET: &str = "https://immutascan.io/address/";
const IMMUTASCAN_TX: &str = "https://immutascan.io/tx/";

pub fn capitalize_label(label: &String) -> String {
    let capitalized_label = match label.get(0..1) {
        Some(first_char) => {
            let mut char = String::from(first_char);
            char.make_ascii_uppercase();
            char
        }
        None => String::new(),
    } + &label[1..];
    capitalized_label
}

pub fn format_number_with_spaces(n: &i64) -> String {
    let mut num_str = n.to_string();
    let num_digits = num_str.len();

    if num_digits <= 3 {
        return num_str;
    }

    let num_groups = (num_digits - 1) / 3;

    for i in 1..=num_groups {
        let insert_pos = num_digits - 3 * i;
        num_str.insert(insert_pos, ' ');
    }

    num_str
}

pub fn format_date(date: NaiveDateTime) -> String {
    Local::from_utc_datetime(&Local, &date)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn format_price(price: &Price) -> Html {
    let price_value = price.price;
    let formatted_price_value = if price_value >= 1000.0 {
        let integer_part = price_value.round() as i64;
        format!("{}", format_number_with_spaces(&integer_part))
    } else if price_value.fract() == 0.0 || (price_value * 100.0).fract() == 0.0 {
        format!("{:.2}", price_value)
    } else {
        format!("{}", price_value)
    };
    let price_html = html!(format!(" {} ", formatted_price_value));
    let currency = price.currency.as_str();
    return match currency {
        "BTC" => {
            html!(
                    <>
                        <i class="fab fa-bitcoin"></i> {price_html}
                    </>)
        }
        "ETH" => {
            html!(
                    <>
                        <i class="fab fa-ethereum"></i> {price_html}
                    </>)
        }
        "USDC" => {
            html!(
                    <>
                        <i class="usdc_icon"></i> {price_html}
                    </>)
        }
        "USD" => {
            html!(
                    <>
                        <i class="fas fa-dollar-sign"></i> {price_html}
                    </>)
        }
        "EUR" => {
            html!(
                    <>
                        <i class="fas fa-euro-sign"></i> {price_html}
                    </>)
        }
        "JPY" => {
            html!(
                    <>
                        <i class="fas fa-yen-sign"></i> {price_html}
                    </>)
        }
        _ => {
            html!({ format!("{} {}", formatted_price_value, currency) })
        }
    };
}

pub fn format_wallet_link(wallet: &String) -> Html {
    fn format_wallet(wallet: &String) -> String {
        if wallet.is_empty() {
            return "".to_string();
        }
        return format!("{}...{}", &wallet[0..5], &wallet[wallet.len() - 4..]);
    }

    if wallet.is_empty() {
        return html!();
    }

    return html!(
        <a href={format!("{}{}", IMMUTASCAN_WALLET, wallet)} target="_blank" class="btn btn-primary me-1">
              { format_wallet(&wallet) }
        </a>);
}

pub fn format_transaction_link(transaction_id: i32, text: String) -> Html {
    html!(
        <a href={format!("{}{}", IMMUTASCAN_TX, transaction_id)} target="_blank" class="text-decoration-none">{ text }</a>
    )
}

pub fn get_asset_link(token_address: &String, token_id: i32, image_url: &String) -> Html {
    html! {
        <Link<Route> to={Route::Asset {token_address: token_address.to_string(), token_id: token_id} }>
            <img src={image_url.clone()} class="img-fluid shadow-gradient" width="50%"
            loading="lazy" alt={token_id.to_string()}/>
        </Link<Route>>
    }
}
