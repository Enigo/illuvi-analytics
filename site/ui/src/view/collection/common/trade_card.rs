use crate::utils::formatting_utils;
use model::model::trade::SingleTrade;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub token_address: String,
    pub trade: SingleTrade,
}

#[function_component(TradeCardWithFlip)]
pub fn get_single_trade_volume_card(props: &CardProps) -> Html {
    // the back side has more information thus I want it to be the height of the card
    let trade = &props.trade;
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
            <div id="flip-card" class={format!("card p-0 {}", *flipped)} {onclick}>
              <div class="card-front">
                  <h5 class="card-header bg-light">{trade.name.clone()}</h5>
                  <div class="card-body bg-pink h-100 d-flex flex-column justify-content-center align-items-center py-3 px-0">
                    <ul class="list-group list-group-flush">
                      <li class="list-group-item bg-pink text-white fs-5 py-3 px-0">{formatting_utils::format_price(&trade.buy_price)}</li>
                      <li class="list-group-item bg-pink text-white fs-5 py-3 px-0">
                        {"From "}{formatting_utils::format_wallet_link(&trade.wallet_from)}{" to "}{formatting_utils::format_wallet_link(&trade.wallet_to)}
                      </li>
                      <li class="list-group-item bg-pink text-white fs-5 py-3 px-0">{formatting_utils::format_date(trade.updated_on)}</li>
                    </ul>
                    <p class="card-text"><small class="text-white">{formatting_utils::format_transaction_link(trade.transaction_id)}</small></p>
                  </div>
              </div>
              <div class="card-back d-flex flex-column">
                    <h5 class="card-header bg-light">{trade.name.clone()}</h5>
                    <div class="card-body text-white bg-pink d-flex flex-column justify-content-center align-items-center py-3 px-0">
                        <p class="card-text fs-2">{formatting_utils::format_price(&trade.usd_price)}</p>
                        {formatting_utils::get_asset_link(&props.token_address, trade.token_id)}
                        <div class="mt-4"><i class="fas fa-undo-alt text-white fs-5"></i></div>
                    </div>
              </div>
            </div>
          </div>
    )
}
