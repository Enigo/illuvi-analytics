use crate::utils::formatting_utils;
use crate::view::common::transactions_view::TransactionsView;
use model::model::asset::CommonOrderData;
use yew::prelude::*;

const ILLUVIDEX: &str = "https://illuvidex.illuvium.io/asset";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub common_order_data: CommonOrderData,
    pub token_address: String,
    pub token_id: i32,
}

#[function_component(AssetOrderData)]
pub fn asset_order_data_function_component(props: &Props) -> Html {
    let common_order_data = &props.common_order_data;
    let token_address = &props.token_address;
    let token_id = &props.token_id;
    let listed = common_order_data.listed_index.is_some();
    html! {
        <div class="row bg-dark text-center my-3 p-5 justify-content-center border border-dark rounded">
            if {listed} {
                <div class="row mb-3">
                    <div class="col">
                        <a href={format!("{}/{}/{}", ILLUVIDEX, token_address, token_id)} target="_blank" class="btn btn-primary me-1">
                               { "Buy on Illuvidex for " } { formatting_utils::format_price(&common_order_data.buy_price.clone().unwrap()) }
                        </a>
                    </div>
                </div>
            }
            <div class="col-md-4 p-0 border rounded bg-dark">
               <ul class="list-group list-group-flush p-2">
                if {listed} {
                  {formatting_utils::get_li_with_span(&String::from("Price Rank"), &common_order_data.listed_index.unwrap())}
                }
                {formatting_utils::get_li_with_span(&String::from("Listed"), &common_order_data.active_orders)}
                {formatting_utils::get_li_with_span(&String::from("Total trades"), &common_order_data.total_filled_orders)}
               </ul>
            </div>
            <div class="row my-4 p-3 justify-content-center border rounded bg-dark">
                <p class="text-white fs-3">{"Cheapest listed"}</p>
                {
                    html!(
                        <TransactionsView transactions={common_order_data.last_active_orders.clone()} token_address={token_address.clone()}/>
                    )
                }
            </div>
            <div class="row p-3 justify-content-center border rounded bg-dark">
                <p class="text-white fs-3">{"Last trades"}</p>
                {
                    html!(
                        <TransactionsView transactions={common_order_data.last_filled_orders.clone()} token_address={token_address.clone()}/>
                    )
                }
            </div>
        </div>
    }
}
