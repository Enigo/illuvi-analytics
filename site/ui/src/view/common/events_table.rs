use crate::utils::{api_utils, formatting_utils, pagination_utils};
use crate::view::loading::LoadingSpinnerDark;
use log::error;
use model::model::transaction::EventData;
use model::model::transaction::TransactionData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
}

#[function_component(EventsTable)]
pub fn events_table_function_component(props: &Props) -> Html {
    let event_types = vec![
        String::from("All"),
        String::from("Burned"),
        String::from("Deposit"),
        String::from("Mint"),
        String::from("Trade active"),
        String::from("Trade cancelled"),
        String::from("Trade filled"),
        String::from("Transfer"),
        String::from("Withdrawal"),
    ];

    let url = &props.url;
    let page = use_state(|| 1);
    let event_index = use_state(|| 0);
    let event_data = use_state(|| None);
    {
        let url = url.clone();
        let event_data = event_data.clone();
        let page = page.clone();
        let page_val = *page;
        let event_index = event_index.clone();
        let event_val = *event_index;
        let event_types = event_types.clone();
        use_effect_with(
            (page_val, event_val, props.url.clone()), move |_| {
                event_data.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<EventData>(
                        format!(
                            "{}&page={}&event={}",
                            url,
                            page_val,
                            &event_types.get(event_val).unwrap_or(&String::from("All"))
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(fetched_data) => {
                            event_data.set(Some(fetched_data));
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
        );
    }

    return html! {
        <div class="container-fluid p-3 bg-dark">
            <div class="col-md-12">
                <p class="text-white text-center fs-2 mb-4">{"Events"}</p>
            </div>
            { events_filter(&event_types, event_index, &page) }
             if let Some(event_data) = (*event_data).as_ref() {
                { pagination_utils::pagination(event_data.total, &page) }
                { render_table(&event_data.transactions) }
             } else {
                <LoadingSpinnerDark />
             }
         </div>
    };
}

fn render_table(transaction_data: &Vec<TransactionData>) -> Html {
    let has_asset_content = transaction_data
        .get(0)
        .map(|data| data.asset_content.is_some())
        .is_some_and(|val| val == true);

    html!(
        <div class="container animate__animated animate__fadeIn animate__faster">
            <div class="row">
                <div class="col-md text-center">
                    <div class="table-responsive">
                        <table class="table text-white border-secondary">
                          <thead>
                            <tr>
                              <th scope="col">{"Event"}</th>
                              <th scope="col">{"From"}</th>
                              <th scope="col">{"To"}</th>
                              if {has_asset_content} {
                                <th scope="col">{"Token"}</th>
                              }
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
                                            if let Some(asset_content) = &transaction.asset_content {
                                                <td scope="row" class="align-middle">
                                                    <span class="d-block mb-1 w-50 mx-auto">{ formatting_utils::get_asset_link(&asset_content.token_address, asset_content.token_id, &asset_content.image_url) }</span>
                                                    <span class="d-block text-white"> { &asset_content.name }</span>
                                                </td>
                                            }
                                            if let Some(price) = &transaction.price {
                                                <td scope="row" class="align-middle">
                                                    <span class="d-block mb-1">{ formatting_utils::format_price(&price) }</span>
                                                    if let Some(usd_price) = &transaction.usd_price {
                                                        <span class="d-block text-muted"> { formatting_utils::format_price(&usd_price) }</span>
                                                    }
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
    )
}

fn events_filter(
    event_types: &Vec<String>,
    event_handler: UseStateHandle<usize>,
    page_handler: &UseStateHandle<i32>,
) -> Html {
    let event_type_label = &String::from("Event Type");
    let empty_label = &String::new();
    let label = if *event_handler == 0 {
        event_type_label
    } else {
        event_types.get(*event_handler).unwrap_or(empty_label)
    };

    return html!(
        <div class="row">
            <div class="col-md text-center">
                <div class="dropdown">
                  <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { label }
                  </button>
                  <ul class="dropdown-menu">
                  {event_types.iter().enumerate().map(|(index, event)| {
                      let event_handler = event_handler.clone();
                      let page_handler = page_handler.clone();
                      html!(
                        <li><button class="dropdown-item" onclick={move |_| {
                              if *event_handler != index {
                                  event_handler.set(index);
                                  page_handler.set(1);
                              }
                          }}>{ event }</button></li>
                      )
                  }).collect::<Html>()}
                  </ul>
                </div>
            </div>
        </div>
    );
}
