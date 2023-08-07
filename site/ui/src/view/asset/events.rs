use crate::utils::{api_utils, formatting_utils};
use log::error;
use model::model::asset::TransactionData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
    pub token_id: i32,
}

#[function_component(AssetEvents)]
pub fn asset_events_function_component(props: &Props) -> Html {
    let transaction_data = use_state(|| vec![]);
    {
        let token_address = props.token_address.clone();
        let token_id = props.token_id;
        let transaction_data = transaction_data.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<Vec<TransactionData>>(
                        format!(
                            "/asset/events?token_address={}&token_id={}",
                            token_address, token_id
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(fetched_data) => {
                            transaction_data.set(fetched_data);
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
            (props.token_id.clone(), props.token_address.clone()),
        );
    }

    html!(
        <div class="container-fluid p-3 bg-dark">
            <div class="container animate__animated animate__fadeIn animate__fast">
                <div class="row">
                    <div class="col-md-12">
                        <p class="text-white text-center fs-2 mb-4">{"Events"}</p>
                    </div>
                    <div class="col-md text-center">
                        <div class="table-responsive">
                            <table class="table text-white border-secondary">
                              <thead>
                                <tr>
                                  <th scope="col">{"Event"}</th>
                                  <th scope="col">{"From"}</th>
                                  <th scope="col">{"To"}</th>
                                  <th scope="col">{"Price"}</th>
                                </tr>
                              </thead>
                              <tbody>
                                {
                                    transaction_data.iter().map(|transaction| {
                                        html!{
                                            <tr key={transaction.updated_on.to_string()}>
                                                if {transaction.id == Option::None} {
                                                    <td scope="row" class="align-middle">
                                                        <span class="d-block mb-1">{ transaction.event.clone() }</span>
                                                        <span class="d-block text-muted"> { formatting_utils::format_date(transaction.updated_on) }</span>
                                                    </td>
                                                } else {
                                                    <td scope="row" class="align-middle">
                                                        <span class="d-block mb-1">{ formatting_utils::format_transaction_link(transaction.id.unwrap(), transaction.event.clone()) }</span>
                                                        <span class="d-block text-muted"> { formatting_utils::format_date(transaction.updated_on) }</span>
                                                    </td>
                                                }
                                                <td class="align-middle">{ formatting_utils::format_wallet_link(&transaction.wallet_from) }</td>
                                                <td class="align-middle">{ formatting_utils::format_wallet_link(&transaction.wallet_to) }</td>
                                                if let Some(price) = &transaction.price {
                                                    <td scope="row" class="align-middle">
                                                        <span class="d-block mb-1">{ formatting_utils::format_price(&price) }</span>
                                                        <span class="d-block text-muted"> { formatting_utils::format_price(&transaction.usd_price.clone().unwrap()) }</span>
                                                    </td>
                                                } else {
                                                    <td></td>
                                                }
                                            </tr>
                                         }
                                    }).collect::<Html>()
                                }
                              </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
