use crate::utils::{api_utils, formatting_utils};
use log::error;
use model::model::asset::LandAssetData;
use yew::prelude::*;

const LAND_ICON: &str = "https://assets.illuvium-game.io/illuvidex/land/land-";

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
            },
            (),
        );
    }

    return match (*asset).as_ref() {
        Some(asset) => {
            html! {
                <section>
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
                </section>
            }
        }
        None => {
            html! {
                <div class="container pt-5">
                    <p class="text-white fs-4 mb-2">{"Loading..."}</p>
                </div>
            }
        }
    };

    fn intro(asset: &LandAssetData) -> Html {
        html!(
            <div class="container-fluid p-5 bg-gray">
                <div class="container animate__animated animate__fadeIn animate__fast">
                    <div class="row g-0">
                      <div class="col-lg-5 order-lg-1">
                        <img src={format!("{}{}{}", LAND_ICON, asset.asset_data.token_id, ".svg")}
                          class="w-100 img-fluid shadow-gradient"
                          loading="lazy" alt={asset.name.clone()}/>
                      </div>
                      <div class="col-lg-7 d-flex align-items-center order-lg-2 text-center text-lg-start p-md-5">
                        <div class="d-flex flex-column">
                          <p class="text-white fs-2 my-2">{asset.name.clone()}</p>
                          <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                          if {asset.landmark != "None"} {
                              <p class="text-white fs-4 mb-2">{format!("Landmark: {}", asset.landmark)}</p>
                          }
                          <p class="text-white fs-4 mb-2">
                              {"Current owner: "}
                              {formatting_utils::format_wallet_link(&asset.asset_data.current_owner)}
                          </p>
                          <p class="text-white fs-4 mb-2">{format!("Last owner change: {}", formatting_utils::format_date(asset.asset_data.last_owner_change))}</p>
                        </div>
                      </div>
                    </div>
                </div>
            </div>
        )
    }

    fn elements(asset: &LandAssetData) -> Html {
        html!(
            <div class="container-fluid p-5 bg-dark">
                <div class="container animate__animated animate__fadeIn animate__fast">
                    <div class="row">
                        <div class="col-12">
                            <p class="text-white text-center fs-2 mb-4">{"Elements"}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/carbon.png" class="img-fluid mb-3 shadow-gradient" alt="Carbon" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Carbon"}</p>
                            <p class="text-white fs-4 mb-4">{asset.carbon}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/hydrogen.png" class="img-fluid mb-3 shadow-gradient" alt="Hydrogen" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Hydrogen"}</p>
                            <p class="text-white fs-4 mb-4">{asset.hydrogen}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/silicon.png" class="img-fluid mb-3 shadow-gradient" alt="Silicon" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Silicon"}</p>
                            <p class="text-white fs-4 mb-4">{asset.silicon}</p>
                        </div>
                    </div>
                </div>
            </div>
        )
    }

    fn fuels(asset: &LandAssetData) -> Html {
        html!(
            <div class="container-fluid px-5 pb-5 bg-dark">
                <div class="container animate__animated animate__fadeIn animate__fast">
                    <div class="row bg-dark">
                        <div class="col-12">
                            <p class="text-white text-center fs-2 mb-4">{"Fuels"}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/crypton.png" class="img-fluid mb-3 shadow-gradient" alt="Crypton" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Crypton"}</p>
                            <p class="text-white fs-4 mb-4">{asset.crypton}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/hyperion.png" class="img-fluid mb-3 shadow-gradient" alt="Hyperion" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Hyperion"}</p>
                            <p class="text-white fs-4 mb-4">{asset.hyperion}</p>
                        </div>
                        <div class="col-md-4 text-center">
                            <img src="/img/solon.png" class="img-fluid mb-3 shadow-gradient" alt="solon" width="25%"/>
                            <p class="text-white fs-4 mb-0">{"Solon"}</p>
                            <p class="text-white fs-4 mb-4">{asset.solon}</p>
                        </div>
                    </div>
                </div>
            </div>
        )
    }

    fn events(asset: &LandAssetData) -> Html {
        html!(
            <div class="container-fluid p-3 bg-gray">
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
                                      <th scope="col">{"Id"}</th>
                                      <th scope="col">{"Event"}</th>
                                      <th scope="col">{"Wallet From"}</th>
                                      <th scope="col">{"Wallet To"}</th>
                                      <th scope="col">{"Price"}</th>
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
                                                        <th scope="row">{formatting_utils::format_transaction_link(transaction.id.unwrap())}</th>
                                                    }
                                                    <td>{ transaction.event.clone() }</td>
                                                    <td>{formatting_utils::format_wallet_link(&transaction.wallet_from)}</td>
                                                    <td>{formatting_utils::format_wallet_link(&transaction.wallet_to)}</td>
                                                    if let Some(price) = &transaction.price {
                                                        <td>{ formatting_utils::format_price(&price) }</td>
                                                    } else {
                                                        <td></td>
                                                    }
                                                    <td>{ formatting_utils::format_date(transaction.updated_on) }</td>
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
}
