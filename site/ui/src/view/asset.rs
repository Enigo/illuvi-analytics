use crate::utils::api_utils;
use log::error;
use model::model::asset::LandAssetData;
use yew::prelude::*;

const LAND_ICON: &str = "https://assets.illuvium-game.io/illuvidex/land/land-";
const IMMUTASCAN_TX: &str = "https://immutascan.io/tx/";
const IMMUTASCAN_WALLET: &str = "https://immutascan.io/address/";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
    pub token_id: i32,
}

#[function_component(AssetLand)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let token_id = props.token_id;
        let asset = asset.clone();
        use_effect_with_deps(
            move |_| {
                let asset = asset.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<LandAssetData>(
                        format!(
                            "/asset/asset?token_address={}&token_id={}",
                            token_address, token_id
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(fetched_asset) => {
                            asset.set(Some(fetched_asset));
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    return match (*asset).as_ref() {
        Some(asset) => {
            html! {
                <div class="container pt-5">
                    {
                        intro(&asset)
                    }
                    {
                        elements(&asset)
                    }
                    {
                        fuels(&asset)
                    }
                    {
                        events(&asset)
                    }
                </div>
            }
        }
        None => {
            html! {
                <>
                    {"No data yet!"}
                </>
            }
        }
    };

    fn intro(asset: &LandAssetData) -> Html {
        html!(
            <div class="row my-3 p-3 g-0 bg-dark">
              <div class="col-lg-5">
                <img src={format!("{}{}{}", LAND_ICON, asset.asset_data.token_id, ".svg")}
                  class="w-100 img-fluid"
                  loading="lazy" alt={asset.name.clone()}/>
              </div>
              <div class="col-lg-7 d-flex align-items-center p-md-5">
                <div class="d-flex flex-column">
                  <p class="text-white fs-2 mb-2">{asset.name.clone()}</p>
                  <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                  if {asset.landmark != "None"} {
                      <p class="text-white fs-4 mb-2">{format!("Landmark: {}", asset.landmark)}</p>
                  }
                  <p class="text-white fs-4 mb-2">
                      {"Current owner: "}
                      <a href={format!("{}{}", IMMUTASCAN_WALLET, asset.asset_data.current_owner.clone())} class="text-decoration-none">
                          { format_wallet(&asset.asset_data.current_owner) }
                      </a>
                  </p>
                  <p class="text-white fs-4 mb-2">{format!("Last owner change: {}", asset.asset_data.last_owner_change)}</p>
                </div>
              </div>
            </div>
        )
    }

    fn elements(asset: &LandAssetData) -> Html {
        html!(
            <div class="row my-3 p-3 bg-dark">
                <div class="col-12">
                    <p class="text-white fs-3 mb-4">{"Elements"}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/carbon.png" class="img-fluid mb-3" alt="Carbon" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Carbon"}</p>
                    <p class="text-white fs-4 mb-4">{asset.carbon}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/hydrogen.png" class="img-fluid mb-3" alt="Hydrogen" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Hydrogen"}</p>
                    <p class="text-white fs-4 mb-4">{asset.hydrogen}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/silicon.png" class="img-fluid mb-3" alt="Silicon" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Silicon"}</p>
                    <p class="text-white fs-4 mb-4">{asset.silicon}</p>
                </div>
            </div>
        )
    }

    fn fuels(asset: &LandAssetData) -> Html {
        html!(
            <div class="row my-3 p-3 bg-dark">
                <div class="col-12">
                    <p class="text-white fs-3 mb-4">{"Fuels"}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/crypton.png" class="img-fluid mb-3" alt="Crypton" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Crypton"}</p>
                    <p class="text-white fs-4 mb-4">{asset.crypton}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/hyperion.png" class="img-fluid mb-3" alt="Hyperion" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Hyperion"}</p>
                    <p class="text-white fs-4 mb-4">{asset.hyperion}</p>
                </div>
                <div class="col-md-4 text-center">
                    <img src="/img/solon.png" class="img-fluid mb-3" alt="solon" width="25%"/>
                    <p class="text-white fs-4 mb-0">{"Solon"}</p>
                    <p class="text-white fs-4 mb-4">{asset.solon}</p>
                </div>
            </div>
        )
    }

    fn events(asset: &LandAssetData) -> Html {
        html!(
            <div class="row my-3 p-3 bg-dark">
                <div class="col-12">
                    <p class="text-white fs-3 mb-4">{"Events"}</p>
                </div>
                <div class="col-md text-center">
                    <table class="table text-white">
                      <thead>
                        <tr>
                          <th scope="col">{"Id"}</th>
                          <th scope="col">{"Event"}</th>
                          <th scope="col">{"Wallet From"}</th>
                          <th scope="col">{"Wallet To"}</th>
                          <th scope="col">{"Date"}</th>
                        </tr>
                      </thead>
                      <tbody>
                        {
                            asset.transaction_data.iter().map(|transaction| {
                                html!{
                                    <tr key={transaction.updated_on.to_string()}>
                                        if {transaction.id == Option::None} {
                                            <th scope="row" />
                                        } else {
                                            <th scope="row"><a href={format!("{}{}", IMMUTASCAN_TX, transaction.id.unwrap())} class="text-decoration-none">{ transaction.id.unwrap() }</a></th>
                                        }
                                        <td>{ transaction.event.clone() }</td>
                                        <td><a href={format!("{}{}", IMMUTASCAN_WALLET, transaction.wallet_from.clone())} class="text-decoration-none">{ format_wallet(&transaction.wallet_from) }</a></td>
                                        <td><a href={format!("{}{}", IMMUTASCAN_WALLET, transaction.wallet_to.clone())} class="text-decoration-none">{ format_wallet(&transaction.wallet_to) }</a></td>
                                        <td>{ transaction.updated_on.to_string() }</td>
                                    </tr>
                                 }
                            }).collect::<Html>()
                        }
                        <tr>
                            <th scope="row"><a href={format!("{}{}", IMMUTASCAN_TX, asset.mint_data.transaction_id)} class="text-decoration-none">{ asset.mint_data.transaction_id }</a></th>
                            <td>{ "Mint" }</td>
                            <td></td>
                            <td ><a href={format!("{}{}", IMMUTASCAN_WALLET, asset.mint_data.wallet.clone())} class="text-decoration-none">{ format_wallet(&asset.mint_data.wallet) }</a></td>
                            <td>{ asset.mint_data.minted_on }</td>
                        </tr>
                      </tbody>
                    </table>
                </div>
            </div>
        )
    }

    fn format_wallet(wallet: &String) -> String {
        if wallet.is_empty() {
            return "".to_string();
        }
        return format!("{}...{}", &wallet[0..6], &wallet[wallet.len() - 4..]);
    }
}
