use crate::utils::formatting_utils;
use model::model::trade::SingleTrade;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewProps {
    pub token_address: String,
    pub trade: SingleTrade,
    pub render_border_end: bool,
}

#[function_component(SingleTradeView)]
pub fn get_single_trade_volume_view(props: &ViewProps) -> Html {
    let trade = &props.trade;
    html!(
          <div class={format!("col-md mb-2 mx-2 p-0 bg-dark {}", if props.render_border_end {"column_with_end_border"} else {""})}>
              <p class="fs-5 text-white m-0 py-2">{trade.name.clone()}</p>
              <div class="justify-content-center align-items-center py-2">
                {formatting_utils::get_asset_link(&props.token_address, trade.token_id, &trade.image_url)}
              </div>
              <p class="text-white fs-5 py-1 m-0">{formatting_utils::format_price(&trade.buy_price)} {" | "} {formatting_utils::format_price(&trade.usd_price)}</p>
              <small>{formatting_utils::format_transaction_link(trade.transaction_id, formatting_utils::format_date(trade.updated_on))}</small>
          </div>
    )
}
