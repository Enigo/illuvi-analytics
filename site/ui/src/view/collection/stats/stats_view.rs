use crate::utils::{api_utils, formatting_utils};
use crate::view::collection::common::{no_data::NoData, trade_card::TradeCardWithFlip};
use crate::view::collection::stats::trade_volume_card::TradeVolumeCardWithFlip;
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
            html! {}
        }
    };
}

fn stats_view(stats_data: &StatsData, token_address: &String) -> Html {
    if stats_data.total.assets == 0 {
        return html!( <NoData /> );
    }

    html! {
        <selection>
            { totals(&stats_data, token_address) }
            { statistics(&stats_data, token_address) }
        </selection>
    }
}

fn totals(stats_data: &StatsData, token_address: &String) -> Html {
    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container">
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
                { render_trades_volume(stats_data) }
                { render_trades_amount_html(stats_data) }
                { render_cheapest_and_most_expensive_trades(stats_data, token_address) }
            </div>
        </div>
    );
}

fn render_totals(total: &StatsDataTotal) -> Html {
    return html! {
        <div class="row p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast">
            <p class="text-white fs-2 mb-2">{"Totals"}</p>
            { formatting_utils::get_single_card(&String::from("Assets"), &String::from("minted"), &total.assets) }
            { formatting_utils::get_single_card(&String::from("Transfers"), &String::from("made"), &total.transfers) }
            { formatting_utils::get_single_card(&String::from("Trades"), &String::from("active | cancelled | filled"), &total.trades) }
        </div>
    };
}

fn render_most(stats_data: &StatsData, token_address: &String) -> Html {
    return html! {
        <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast">
            if !&stats_data.most_transferred_token.is_empty() {
                { html!(get_single_most_token_card(String::from("Most Transferred Token"), token_address, &stats_data.most_transferred_token)) }
            }
            if !&stats_data.most_traded_token.is_empty() {
                { html!(get_single_most_token_card(String::from("Most Traded Token"), token_address, &stats_data.most_traded_token)) }
            }
            if !&stats_data.most_traded_wallet.is_empty() {
                { html!(get_single_most_wallet_card(String::from("Most Trading Wallet"), &stats_data.most_traded_wallet)) }
            }
        </div>
    };
}

fn render_trades_volume(stats_data: &StatsData) -> Html {
    let trades_volume = &stats_data.trades_volume;
    return html!(
        <div class="row text-center animate__animated animate__fadeIn animate__fast">
            <p class="text-white fs-2 mb-0">{"Trades"}</p>
            <p class="text-white fs-3 mb-2">{"Volume"}</p>
            <p class="text-white fs-5 mb-0">{"There are various cryptocurrencies that can be used to purchase the assets"}</p>
            <p class="text-white fs-5 mb-0">{"This section lists all of them conveniently converted into multiple other currencies"}</p>
            <div class="row row-cols-1 row-cols-md-3 g-3 justify-content-center mb-5 mt-1">
                {
                    trades_volume.iter().map(|volume| {
                        let volume = volume.clone();
                        html! { <TradeVolumeCardWithFlip {volume}/> }
                    }).collect::<Html>()
                }
            </div>
        </div>
    );
}

fn render_trades_amount_html(stats_data: &StatsData) -> Html {
    let trades_by_status = &stats_data.trades_by_status;
    return html!(
        <div class="row text-center mb-5 animate__animated animate__fadeIn animate__fast">
            <p class="text-white text-center fs-3 mb-2">{"Amount"}</p>
            { trades_by_status.iter().map(|(status, trades)| {
                let mut total_per_status = 0;
                html!(
                    <div class="col-md mb-2">
                        <div class="card">
                          <p class="card-header fs-4">{formatting_utils::capitalize_label(status)}</p>
                          <ul class="list-group list-group-flush">
                            { trades.iter().rev().map(|trade| {
                                total_per_status += trade.count;
                                html!(
                                    { get_single_li(&trade.buy_currency, &trade.count) }
                                )
                            }).collect::<Html>() }
                          </ul>
                          <p class="card-header fs-4">{format!("Total {}", formatting_utils::format_number_with_spaces(&total_per_status))}</p>
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
    let trades_by_tier = &stats_data.cheapest_and_most_expensive_trades_by_tier;
    return html! {
        <div class="row my-3 p-3 text-center justify-content-center animate__animated animate__fadeIn animate__fast">
            <p class="text-white fs-3 mb-2">{"Cheapest | Most Expensive"}</p>
            { trades_by_tier.iter().map(|(tier, trades)| {
                html! {
                     <div class="row text-center mb-5">
                        <p class="text-white fs-4 mb-2">{format!("Tier {}", tier)}</p>
                        { trades.iter().map(|trade| {
                            let trade = trade.clone();
                            let token_address = token_address.clone();
                            html!( <TradeCardWithFlip {token_address} {trade}/> )
                        }).collect::<Html>() }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    };
}

fn get_single_li(label: &String, total: &i64) -> Html {
    html!(
      <li class="list-group-item bg-pink text-white fs-5 d-flex justify-content-between align-items-center w-100">
        <span class="badge bg-primary">{label}</span>
        {formatting_utils::format_number_with_spaces(total)}
      </li>
    )
}

fn get_single_most_token_card(
    title: String,
    token_address: &String,
    most_data: &Vec<StatsDataMostEventForToken>,
) -> Html {
    html!(
          <div class="col-md mb-2">
            <div class="card">
              <h5 class="card-header">{title}</h5>
              <div class="card-body bg-pink text-white">
                <p class="card-text fs-4 mb-0">{most_data[0].count}</p>
                <p class="card-text"><small class="text-white">{"times"}</small></p>
                { most_data.iter().map(|data| {
                    formatting_utils::get_asset_link(token_address, data.token_id)
                }).collect::<Html>() }
              </div>
            </div>
          </div>
    )
}

fn get_single_most_wallet_card(
    title: String,
    most_data: &Vec<StatsDataMostEventForWallet>,
) -> Html {
    html!(
          <div class="col-md mb-2">
            <div class="card">
              <h5 class="card-header">{title}</h5>
              <div class="card-body bg-pink text-white">
                <p class="card-text fs-4 mb-0">{most_data[0].count}</p>
                <p class="card-text"><small class="text-white">{"times"}</small></p>
                { most_data.iter().map(|data| {
                    html! {
                        formatting_utils::format_wallet_link(&data.wallet)
                    }
                }).collect::<Html>() }
              </div>
            </div>
          </div>
    )
}
