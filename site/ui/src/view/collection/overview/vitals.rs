use crate::utils::{api_utils, formatting_utils};
use crate::view::collection::common::{no_data::NoData, trade_card::TradeCardWithFlip};
use log::error;
use model::model::price::Price;
use model::model::vitals::{VitalsData, VitalsDataFloor};
use std::collections::BTreeMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionVitals)]
pub fn collection_mint_function_component(props: &Props) -> Html {
    let vitals = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let stats = vitals.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<VitalsData>(
                        format!("/stat/vitals?token_address={}", token_address).as_str(),
                    )
                    .await
                    {
                        Ok(fetched_data) => {
                            stats.set(Some(fetched_data));
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
            props.token_address.clone(),
        );
    }

    return match (*vitals).as_ref() {
        Some(vitals_data) => {
            html! {
                <selection>
                    { vitals_view(&vitals_data, &props.token_address) }
                </selection>
            }
        }
        None => {
            html! {}
        }
    };
}

fn vitals_view(vitals_data: &VitalsData, token_address: &String) -> Html {
    if vitals_data.floor.is_empty() {
        return html!( <NoData /> );
    }

    fn get_single_floor_card(data_floor: &VitalsDataFloor, token_address: &String) -> Html {
        html!(
          <div class="col-4">
            <div class="card">
              <h5 class="card-header">{&data_floor.name}</h5>
              <div class="card-body bg-pink text-white">
                <p class="card-text fs-4">{formatting_utils::format_price(&data_floor.price)}</p>
                <Link<Route> to={Route::Asset {token_address: token_address.clone(), token_id: data_floor.token_id} } classes="btn btn-primary">
                    { format!("Token {}", data_floor.token_id) }
                </Link<Route>>
              </div>
            </div>
          </div>
        )
    }

    let mut grouped_data = BTreeMap::new();
    for data in &vitals_data.floor {
        grouped_data
            .entry(data.tier)
            .or_insert_with(Vec::new)
            .push(data);
    }

    let floor_data_html = grouped_data.iter().map(|(tier, data_floors)|
        html!(
            <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast animate__delay-0.25s">
                <p class="text-white fs-3">{format!("Tier {} floors", tier)}</p>
                { data_floors.iter().map(|data_floor|get_single_floor_card(data_floor, token_address)).collect::<Html>()}
            </div>
        )
    ).collect::<Html>();

    let last_trades = &vitals_data.last_trades;
    let last_trades_html = html! {
        <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast animate__delay-0.25s">
            <p class="text-white fs-3 mb-2">{format!("Last {} trades", last_trades.len())}</p>
             <div class="row text-center mb-5">
                { last_trades.iter().map(|trade| {
                    let trade = trade.clone();
                    let token_address = token_address.clone();
                    html!( <TradeCardWithFlip {token_address} {trade}/> )
                }).collect::<Html>() }
            </div>
        </div>
    };

    let trades_volume = vitals_data.trades_volume.clone();
    return html! {
        <div class="container-fluid p-5 bg-gray">
            <div class="container">
                <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast animate__delay-0.25s">
                    { formatting_utils::get_single_card(&String::from("Assets"), &String::from("minted"), &vitals_data.total_assets) }
                    { formatting_utils::get_single_card(&String::from("Unique holders"), &String::from("wallets"), &vitals_data.unique_holders) }
                    { html! { <CardWithOnClick {trades_volume} /> } }
                </div>
                { floor_data_html }
                { last_trades_html }
            </div>
        </div>
    };
}

#[derive(Properties, PartialEq)]
struct CardProps {
    trades_volume: Vec<Price>,
}

#[function_component(CardWithOnClick)]
fn card_with_onlick(props: &CardProps) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        let len = props.trades_volume.len();
        Callback::from(move |_| {
            let next_value = if len - 1 == *counter { 0 } else { *counter + 1 };
            counter.set(next_value)
        })
    };
    let volume = &props.trades_volume;
    html!(
          <div class="col-4">
            // summary adds pointer cursor
            <summary class="card" {onclick}>
              <h5 class="card-header">{"Total trades"}</h5>
              <div class="card-body bg-pink text-white">
                 <p class="card-text fs-4 mb-0">{ formatting_utils::format_price(&volume[*counter]) }</p>
                 <p class="card-text"><small class="text-white">{"across all cryptocurrencies"}</small></p>
              </div>
            </summary>
          </div>
    )
}
