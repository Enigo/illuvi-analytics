use crate::utils::formatting_utils::format_number_with_spaces;
use crate::utils::{api_utils, formatting_utils, navigation_utils};
use crate::view::collection::common::{no_data::NoData, trade_view::SingleTradeView};
use crate::view::loading::LoadingSpinnerGray;
use log::error;
use model::model::stats::{
    StatsData, StatsDataMostEventForToken, StatsDataMostEventForWallet, StatsDataTotal,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionStatsView)]
pub fn stats_view_function_component(props: &Props) -> Html {
    let stats = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let stats = stats.clone();
        use_effect_with_deps(
            move |_| {
                stats.set(None);
                navigation_utils::scroll_to_top();
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<StatsData>(
                        format!("/stat/stats?token_address={}", token_address).as_str(),
                    )
                    .await
                    {
                        Ok(fetched_mint) => {
                            stats.set(Some(fetched_mint));
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

    return match (*stats).as_ref() {
        Some(stats_data) => {
            html!({ stats_view(stats_data, &props.token_address) })
        }
        None => {
            html! {
                <LoadingSpinnerGray />
            }
        }
    };
}

fn stats_view(stats_data: &StatsData, token_address: &String) -> Html {
    if stats_data.total.assets_minted == 0 {
        return html!( <NoData /> );
    }

    html! {
        <selection>
            { totals(&stats_data, token_address) }
            { statistics(&stats_data, token_address) }
            { trades(&stats_data) }
        </selection>
    }
}

fn totals(stats_data: &StatsData, token_address: &String) -> Html {
    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container p-0">
                { render_totals(&stats_data.total) }
                { render_most(stats_data, token_address) }
            </div>
        </div>
    )
}

fn statistics(stats_data: &StatsData, token_address: &String) -> Html {
    return html!(
        <div class="container-fluid p-5 pt-1 bg-dark">
            <div class="container mt-4">
                { render_cheapest_and_most_expensive_trades(stats_data, token_address) }
            </div>
        </div>
    );
}

fn trades(stats_data: &StatsData) -> Html {
    return html!(
        <div class="container-fluid p-5 pt-1 bg-gray">
            <div class="container mt-4">
                { render_trades_volume(stats_data) }
                { render_trades_amount(stats_data) }
            </div>
        </div>
    );
}

fn render_totals(total: &StatsDataTotal) -> Html {
    let burn_rate = total.assets_burnt as f64 / total.assets_minted as f64 * 100.0;
    return html! {
        <div class="row text-center justify-content-center">
            <p class="text-white fs-3 mb-2">{"Totals"}</p>
            <div class="col-md-4 p-0 m-2 border rounded bg-dark">
               <ul class="list-group list-group-flush p-2">
                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                      <span class="badge bg-primary">{"Assets"}</span>{ format_number_with_spaces(&total.assets_minted) }</li>
                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                      <span class="badge bg-primary">{"Burnt"}</span>{ format_number_with_spaces(&total.assets_burnt) }</li>
                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                      <span class="badge bg-primary">{"Burn Rate"}</span>{ format!("{:.2}%", burn_rate) }</li>
               </ul>
            </div>
            <div class="col-md-4 p-0 m-2 border rounded bg-dark">
               <ul class="list-group list-group-flush p-2">
                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                      <span class="badge bg-primary">{"Transfers"}</span>{ &format_number_with_spaces(&total.transfers) }</li>
                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                      <span class="badge bg-primary">{"Trades"}</span>{ &format_number_with_spaces(&total.trades) }</li>
               </ul>
            </div>
        </div>
    };
}

fn render_most(stats_data: &StatsData, token_address: &String) -> Html {
    return html! {
        <>
            if !&stats_data.most_transferred_tokens.is_empty() {
                { html!(get_single_most_assets_view(String::from("Most Transferred Assets"), token_address, &stats_data.most_transferred_tokens)) }
            }
            if !&stats_data.most_traded_tokens.is_empty() {
                { html!(get_single_most_assets_view(String::from("Most Traded Assets"), token_address, &stats_data.most_traded_tokens)) }
            }
            if !&stats_data.most_trading_wallets.is_empty() {
                { html!(get_single_most_wallets_view(String::from("Most Trading Wallets"), &stats_data.most_trading_wallets)) }
            }
        </>
    };
}

fn render_trades_volume(stats_data: &StatsData) -> Html {
    let trades_volume = &stats_data.trades_volume;
    return html!(
        <div class="row justify-content-center animate__animated animate__fadeIn animate__faster">
            <p class="text-white text-center fs-2 mb-1">{"Trades"}</p>
            <div class="row text-center justify-content-center bg-dark border rounded p-2">
                <p class="text-white fs-3 mb-2">{"Volume"}</p>
                <p class="text-white fs-5 mb-0">{"There are various cryptocurrencies that can be used to purchase the assets"}</p>
                <p class="text-white fs-5 mb-0">{"This section lists all of them conveniently converted into multiple other currencies"}</p>
                {
                    trades_volume.iter().map(|volume| {
                        html! {
                          <div class={format!("col-md-2 p-2 m-2 border rounded")}>
                            <p class="text-white fs-5"><strong>{format!("{} in {}", formatting_utils::format_number_with_spaces(&volume.total_trades), volume.total_in_buy_currency.currency)}</strong></p>
                            <p class="text-white fs-5"><strong>{formatting_utils::format_price(&volume.total_in_buy_currency)}</strong></p>
                            <i class="fas fa-equals fs-3 text-white"></i>
                            <ul class="list-group list-group-flush">
                              { volume.totals_in_other_currency.iter().map(|price|
                              html!(
                                    <li class="list-group-item bg-dark text-white fs-5">{formatting_utils::format_price(&price)}</li>
                                  )).collect::<Html>() }
                            </ul>
                          </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    );
}

fn render_trades_amount(stats_data: &StatsData) -> Html {
    let trades_by_status = &stats_data.trades_by_status;
    return html!(
        <div class="row text-center my-4 p-2 bg-dark border rounded animate__animated animate__fadeIn animate__faster">
            <p class="text-white text-center fs-3 mb-2">{"Amount"}</p>
            { trades_by_status.iter().enumerate().map(|(index, (status, trades))| {
                let mut total_per_status = 0;
                html!(
                    <div class={format!("col-md mb-2 {}", if index < trades_by_status.len() - 1 {"column_with_end_border"} else {""})}>
                        <div class="d-flex flex-column h-100">
                          <p class="fs-4 text-white">{formatting_utils::capitalize_label(status)}</p>
                          <ul class="list-group list-group-flush flex-grow-1">
                            { trades.iter().rev().map(|trade| {
                                total_per_status += trade.count;
                                html!(
                                  <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                                    <span class="badge bg-primary">{&trade.buy_currency}</span>
                                    {formatting_utils::format_number_with_spaces(&trade.count)}
                                  </li>
                                )
                            }).collect::<Html>() }
                          </ul>
                          <p class="fs-4 text-white mt-auto">{format!("Total {}", formatting_utils::format_number_with_spaces(&total_per_status))}</p>
                        </div>
                    </div>
                )}).collect::<Html>()
            }
        </div>
    );
}

fn render_cheapest_and_most_expensive_trades(
    stats_data: &StatsData,
    token_address: &String,
) -> Html {
    let trades_by_attribute = &stats_data.cheapest_and_most_expensive_trades_by_attribute;
    return html! {
        <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__faster">
            <p class="text-white fs-3 mb-2">{"Cheapest | Most Expensive Trades"}</p>
            { trades_by_attribute.iter().map(|(attribute, trades)| {
                html! {
                     <div class="row text-center mb-5 p-3 bg-dark border rounded">
                        <p class="text-white fs-4 mb-2">{attribute}</p>
                        { trades.iter().enumerate().map(|(index, trade)| {
                            let trade = trade.clone();
                            let token_address = token_address.clone();
                            let render_border_end = index < trades.len() - 1;
                            html!( <SingleTradeView {token_address} {trade} {render_border_end}/> )
                        }).collect::<Html>() }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    };
}

fn get_single_most_assets_view(
    title: String,
    token_address: &String,
    most_data: &Vec<StatsDataMostEventForToken>,
) -> Html {
    html!(
        <div class="row bg-dark text-center my-4 p-3 justify-content-center border rounded animate__animated animate__fadeIn animate__faster">
            <p class="fs-4 mb-0 text-white">{title}</p>
            <p class="fs-5 text-white">{format!("{} times", most_data[0].count)}</p>
              { most_data.iter().map(|data| {
                  html! {
                      <div class="col-md-3 mb-2 mx-1 p-2 border border-muted rounded bg-dark">
                          <p class="fs-5 text-white m-0 py-2">{data.name.clone()}</p>
                          {formatting_utils::get_asset_link(token_address, data.token_id, &data.image_url)}
                      </div>
                  }
              }).collect::<Html>() }
        </div>
    )
}

fn get_single_most_wallets_view(
    title: String,
    most_data: &Vec<StatsDataMostEventForWallet>,
) -> Html {
    html!(
        <div class="row bg-dark text-center my-4 p-3 justify-content-center border rounded animate__animated animate__fadeIn animate__faster">
          <p class="fs-4 mb-0 text-white">{title}</p>
          <ul class="list-group list-group-flush p-2">
            { most_data.iter().map(|data| {
                html! {
                    <li class="list-group-item bg-dark text-white fs-5 d-flex justify-content-between align-items-center w-100">
                        <div class="col-md text-end me-2">
                            {formatting_utils::format_wallet_link(&data.wallet)}
                        </div>
                        <div class="col-md text-start ms-2">
                            {format!("{} times", data.count)}
                        </div>
                    </li>
                }
            }).collect::<Html>() }
          </ul>
      </div>
    )
}
