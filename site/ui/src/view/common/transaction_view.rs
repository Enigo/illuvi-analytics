use crate::utils::formatting_utils;
use model::model::transaction::SingleTransaction;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewProps {
    pub token_address: String,
    pub trade: SingleTransaction,
    pub render_border_end: bool,
}

#[function_component(SingleTransactionView)]
pub fn single_transaction_view(props: &ViewProps) -> Html {
    let transaction = &props.trade;
    let transaction_id = transaction.transaction_id;
    html!(
          <div class={format!("col-md-4 mb-2 p-0 bg-dark {}", if props.render_border_end {"column_with_end_border"} else {""})}>
              <p class="fs-5 text-white m-0 py-2">{transaction.name.clone()}</p>
              <div class="justify-content-center align-items-center py-2">
                {formatting_utils::get_asset_link(&props.token_address, transaction.token_id, &transaction.image_url)}
              </div>
              <p class="text-white fs-5 py-1 m-0">{formatting_utils::format_price(&transaction.buy_price)} {" | "} {formatting_utils::format_price(&transaction.usd_price)}</p>
              if let Some(transaction_id) = transaction_id {
                 <small> { formatting_utils::format_transaction_link(transaction_id, formatting_utils::format_date(transaction.updated_on)) } </small>
              } else {
                 <small class="text-white"> { formatting_utils::format_date(transaction.updated_on) } </small>
              }
          </div>
    )
}
