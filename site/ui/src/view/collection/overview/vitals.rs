use crate::utils::{api_utils, formatting_utils};
use crate::view::collection::common::{no_data::NoData, trade_card::TradeCardWithFlip};
use log::error;
use model::model::price::Price;
use model::model::vitals::{AttributeData, VitalsData, VitalsDataFloor};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::utils::formatting_utils::format_number_with_spaces;

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
    if vitals_data.data_by_attribute.is_empty() {
        return html!( <NoData /> );
    }

    let attribute_data_html = vitals_data.data_by_attribute.iter().map(|(attribute, attribute_data)|
        html!(
            <div class="row text-center my-3 pb-3 justify-content-center animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
                <p class="text-white fs-3">{attribute}</p>
                <div class="row rounded border justify-content-center p-3">
                    {
                        get_single_attribute_card(attribute_data)
                    }
                    <p class="text-white fs-4 pt-3">{ "Floors" }</p>
                    { attribute_data.floor.iter().map(|data_floor|get_single_floor_card(data_floor, token_address)).collect::<Html>()}
                </div>
            </div>
        )
    ).collect::<Html>();

    let last_trades = &vitals_data.last_trades;
    let last_trades_html = html! {
        <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
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
                <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
                    { formatting_utils::get_single_card(&String::from("Assets"), &String::from("minted"), &format_number_with_spaces(&vitals_data.total_assets)) }
                    { formatting_utils::get_single_card(&String::from("Unique holders"), &String::from("wallets"), &format_number_with_spaces(&vitals_data.unique_holders)) }
                    { html! { <CardWithOnClick {trades_volume} /> } }
                </div>
                { last_trades_html }
                { attribute_data_html }
            </div>
        </div>
    };
}

fn get_single_floor_card(data_floor: &VitalsDataFloor, token_address: &String) -> Html {
    html!(
      <div class="col-md-4 mb-2">
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

fn get_single_attribute_card(data: &AttributeData) -> Html {
    let minted_burnt = &data.minted_burnt;
    let available_assets = minted_burnt.total_minted.clone() - minted_burnt.total_burnt.clone();
    let listed_rate = if available_assets == 0 {
        0_f64
    } else {
        data.active_orders.clone() as f64 / available_assets as f64 * 100.0
    };
    let burn_rate =
        minted_burnt.total_burnt.clone() as f64 / minted_burnt.total_minted.clone() as f64 * 100.0;
    html!(
      <div class="col-md-4 mb-2">
        <div class="card">
          <div class="card-body bg-pink text-white">
             <ul class="list-group list-group-flush">
                <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
                    <span class="badge bg-primary">{"Listed"}</span>{ format_number_with_spaces(&data.active_orders) }</li>
                <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
                    <span class="badge bg-primary">{"Listed Rate"}</span>{ format!("{:.2}%", listed_rate) }</li>
                <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
                    <span class="badge bg-primary">{"Minted"}</span>{ format_number_with_spaces(&minted_burnt.total_minted) }</li>
                <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
                    <span class="badge bg-primary">{"Burnt"}</span> { format_number_with_spaces(&minted_burnt.total_burnt) }</li>
                <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
                    <span class="badge bg-primary">{"Burn Rate"}</span>{ format!("{:.2}%", burn_rate) }</li>
            </ul>
          </div>
        </div>
      </div>
    )
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
          <div class="col-md mb-2">
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
