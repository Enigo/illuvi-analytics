use crate::utils::formatting_utils;
use model::model::stats::StatsDataTradesVolume;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub volume: StatsDataTradesVolume,
}

#[function_component(TradeVolumeCardWithFlip)]
pub fn get_single_trade_volume_card(props: &CardProps) -> Html {
    // the back side has more information thus I want it to be the height of the card
    let volume = &props.volume;
    let flipped = use_state(|| String::from("flipped"));
    let onclick = {
        let flipped = flipped.clone();
        Callback::from(move |_| {
            let next_value = if flipped.is_empty() {
                String::from("flipped")
            } else {
                String::new()
            };
            flipped.set(next_value)
        })
    };

    html!(
        <div class="col">
            <div id="flip-card" class={format!("card bg-pink p-0 {}", *flipped)} {onclick}>
                  <div class="card-front">
                      <div class="card-body bg-pink h-100">
                        <p class="text-white fs-5">{"Equivalent to"}</p>
                        <ul class="list-group list-group-flush">
                          { volume.totals_in_other_currency.iter().map(|price|
                          html!(
                                <li class="list-group-item bg-pink text-white fs-5">{formatting_utils::format_price(&price)}</li>
                              )).collect::<Html>() }
                        </ul>
                      </div>
                  </div>
                  <div class="card-back">
                      <div class="card-body bg-pink h-100 d-flex flex-column justify-content-center align-items-center ">
                        <p class="text-white fs-2">{format!("{} trades", formatting_utils::format_number_with_spaces(&volume.total_trades))}</p>
                        <p class="text-white fs-2">{formatting_utils::format_price(&volume.total_in_buy_currency)}</p>
                        <i class="fas fa-undo-alt text-white fs-5"></i>
                      </div>
                  </div>
            </div>
          </div>
    )
}
