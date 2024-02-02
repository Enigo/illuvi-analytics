use crate::utils::{api_utils, formatting_utils, navigation_utils};
use crate::view::loading::LoadingSpinnerGray;
use log::error;
use model::model::wallet::{TotalPerCollectionData, WalletData, WalletMoneyData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub wallet: String,
}

#[function_component(WalletDataView)]
pub fn wallet_data_view_function_component(props: &Props) -> Html {
    let wallet_data = use_state(|| None);
    {
        let wallet = props.wallet.clone();
        let wallet_data = wallet_data.clone();
        use_effect_with(props.wallet.clone(), move |_| {
            wallet_data.set(None);
            navigation_utils::scroll_to_top();
            wasm_bindgen_futures::spawn_local(async move {
                match api_utils::fetch_single_api_response::<WalletData>(
                    format!("/wallet/wallet?wallet={}", wallet).as_str(),
                )
                .await
                {
                    Ok(fetched_data) => {
                        wallet_data.set(Some(fetched_data));
                    }
                    Err(e) => {
                        error!("{e}")
                    }
                }
            });
        });
    }

    return match (*wallet_data).as_ref() {
        Some(data) => {
            return html! {
                { wallet_view(data) }
            };
        }
        None => {
            html! {
                <LoadingSpinnerGray />
            }
        }
    };
}

fn wallet_view(data: &WalletData) -> Html {
    let minted_per_collection_wallet = &data.minted_per_collection_wallet;
    let owned_per_collection_wallet = &data.owned_per_collection_wallet;
    let money_data = &data.money_data;

    html! {
        <div class="container-fluid p-3 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                <div class="row">
                    <p class="text-white text-break fs-2 my-2">{data.wallet.clone()}</p>
                </div>
                <div class="row justify-content-center text-center">
                    {html! {wallet_mint_view(minted_per_collection_wallet)}}
                    {html! {wallet_asset_view(owned_per_collection_wallet)}}
                    {html! {wallet_money_view(money_data)}}
                </div>
            </div>
        </div>
    }
}

fn wallet_mint_view(minted_per_collection_wallet: &Vec<TotalPerCollectionData>) -> Html {
    let total_per_wallet_sum = minted_per_collection_wallet
        .iter()
        .map(|mint| mint.total_per_wallet)
        .sum();

    html! {
       <div class="col-md-3 p-0 m-2 border rounded bg-dark">
           <ul class="list-group list-group-flush p-2">
            { formatting_utils::get_li_with_span(&String::from("Total minted"), &total_per_wallet_sum) }
            { minted_per_collection_wallet.iter().map(|mint|
                html!(
                     formatting_utils::get_li_with_span(&mint.name.clone(), &mint.total_per_wallet)
                )).collect::<Html>()
            }
           </ul>
       </div>
    }
}

fn wallet_asset_view(owned_per_collection_wallet: &Vec<TotalPerCollectionData>) -> Html {
    let total_per_wallet_sum = owned_per_collection_wallet
        .iter()
        .map(|mint| mint.total_per_wallet)
        .sum();

    html! {
       <div class="col-md-3 p-0 m-2 border rounded bg-dark">
           <ul class="list-group list-group-flush p-2">
            { formatting_utils::get_li_with_span(&String::from("Owned"), &total_per_wallet_sum) }
            { owned_per_collection_wallet.iter().map(|asset|
                html!(
                     formatting_utils::get_li_with_span(&asset.name.clone(), &asset.total_per_wallet)
                )).collect::<Html>()
            }
           </ul>
       </div>
    }
}

fn wallet_money_view(money_data: &WalletMoneyData) -> Html {
    html! {
       <div class="col-md-3 p-0 m-2 border rounded bg-dark">
           <ul class="list-group list-group-flush p-2">
            { formatting_utils::get_li_with_span_and_price(&String::from("Total spent on mint"), &money_data.mint_spend_usd) }
            { formatting_utils::get_li_with_span_and_price(&String::from("Total bought"), &money_data.total_buy_usd) }
            { formatting_utils::get_li_with_span_and_price(&String::from("Total sold"), &money_data.total_sell_usd) }
            { formatting_utils::get_li_with_span_and_price(&String::from("Total on sale"), &money_data.total_active_usd) }
            { formatting_utils::get_li_with_span(&String::from("Listed"), &money_data.total_active) }
           </ul>
       </div>
    }
}
