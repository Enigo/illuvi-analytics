use crate::utils::{api_utils, formatting_utils};
use crate::view::common::{no_data::NoData, transactions_view::TransactionsView};
use crate::view::loading::LoadingSpinnerGray;
use log::error;
use model::model::vitals::{AttributeData, VitalsData, VitalsDataFloor};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionVitals)]
pub fn collection_mint_function_component(props: &Props) -> Html {
    let vitals = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let vitals = vitals.clone();
        use_effect_with(props.token_address.clone(), move |_| {
            vitals.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match api_utils::fetch_single_api_response::<VitalsData>(
                    format!("/stat/vitals?token_address={}", token_address).as_str(),
                )
                .await
                {
                    Ok(fetched_data) => {
                        vitals.set(Some(fetched_data));
                    }
                    Err(e) => {
                        error!("{e}")
                    }
                }
            });
        });
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
            html! {
                <LoadingSpinnerGray />
            }
        }
    };
}

fn vitals_view(vitals_data: &VitalsData, token_address: &String) -> Html {
    if vitals_data.data_by_attribute.is_empty() {
        return html!( <NoData /> );
    }

    let attribute_data_html = vitals_data.data_by_attribute.iter().map(|(attribute, attribute_data)|
        html!(
            <div class="row bg-dark text-center my-4 p-3 justify-content-center border rounded animate__animated animate__fadeIn animate__faster">
                <p class="text-white fs-3">{attribute}</p>
                {
                    get_single_attribute_view(attribute_data)
                }
                if {!attribute_data.floor.is_empty()} {
                    <p class="text-white fs-4 pt-3">{ "Floors by Crypto" }</p>
                    { attribute_data.floor.iter().map(|data_floor|get_single_floor_view(data_floor, token_address)).collect::<Html>()}
                }
            </div>
        )
    ).collect::<Html>();

    let last_trades = &vitals_data.last_trades;
    let last_trades_html = html! {
        <div class="row bg-dark text-center my-3 p-3 justify-content-center border rounded animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
            <p class="text-white fs-3 mb-2">{"Last trades"}</p>
            {
                html!(
                    <TransactionsView transactions={last_trades.clone()} token_address={token_address.clone()}/>
                )
            }
        </div>
    };

    let trades_volume = vitals_data.trades_volume.clone();
    let totals_html = html! {
        <div class="row text-center justify-content-center">
            <div class="col-md-4 p-0 m-2 border rounded bg-dark">
               <ul class="list-group list-group-flush p-2">
                  { formatting_utils::get_li_with_span(&String::from("Assets"), &vitals_data.total_assets) }
                  { formatting_utils::get_li_with_span(&String::from("Owners"), &vitals_data.unique_holders) }
               </ul>
            </div>
            <div class="col-md-4 p-0 m-2 border rounded bg-dark">
               <ul class="list-group list-group-flush p-2">
                  { trades_volume.iter().map(|price| {
                      html!(
                          { formatting_utils::get_li_with_span_and_price(&{format!("Sell Volume in {}", price.currency)}, price) }
                      )
                  }).collect::<Html>() }
               </ul>
            </div>
        </div>
    };

    return html! {
        <div class="container-fluid p-5 bg-gray">
            <div class="container">
                { totals_html }
                { last_trades_html }
                { attribute_data_html }
            </div>
        </div>
    };
}

fn get_single_floor_view(data_floor: &VitalsDataFloor, token_address: &String) -> Html {
    html!(
      <div class="col-md-3 mb-2 mx-1 p-0 border border-muted rounded bg-dark">
          <p class="fs-5 text-white m-0 py-2">{&data_floor.name}</p>
          <div class="justify-content-center align-items-center py-2">
            { formatting_utils::get_asset_link(token_address, data_floor.token_id, &data_floor.image_url) }
          </div>
          <p class="text-white fs-5 py-1 m-0">{formatting_utils::format_price(&data_floor.price)} {" | "} {formatting_utils::format_price(&data_floor.usd_price)}</p>
      </div>
    )
}

fn get_single_attribute_view(data: &AttributeData) -> Html {
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
      <div class="col-md-4 p-0 border rounded bg-dark">
         <ul class="list-group list-group-flush p-2">
            { formatting_utils::get_li_with_span(&String::from("Listed"), &data.active_orders) }
            { formatting_utils::get_li_with_span_and_text(&String::from("Listed Rate"), &format!("{:.2}%", listed_rate)) }
            { formatting_utils::get_li_with_span(&String::from("Minted"), &minted_burnt.total_minted) }
            { formatting_utils::get_li_with_span(&String::from("Burnt"), &minted_burnt.total_burnt) }
            { formatting_utils::get_li_with_span_and_text(&String::from("Burn Rate"), &format!("{:.2}%", burn_rate)) }
         </ul>
      </div>
    )
}
