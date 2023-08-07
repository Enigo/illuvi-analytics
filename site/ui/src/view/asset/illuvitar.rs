use crate::route::Route;
use crate::utils::formatting_utils;
use model::model::asset::IlluvitarAssetData;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub illuvitar: IlluvitarAssetData,
}

#[function_component(AssetIlluvitar)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = &props.illuvitar;
    return html! {
        <section>
            {
                intro(&asset)
            }
        </section>
    };
}

fn intro(asset: &IlluvitarAssetData) -> Html {
    let burned = asset.common_asset_data.burned.clone();
    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__fast">
                <div class="row g-0">
                  <div class="col-lg-5 order-lg-1 d-flex align-items-center justify-content-center text-center">
                    <img src={asset.common_asset_data.image_url.clone()}
                      class={format!("w-75 img-fluid shadow-gradient {}", if {burned} {"grayscale"} else {""})}
                      loading="lazy" alt={asset.common_asset_data.name.clone()}/>
                  </div>
                  <div class="col-lg-7 d-flex align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start p-md-5">
                    <div class="d-flex flex-column">
                      <div class="row align-items-center">
                        <div class="col-md-auto">
                          <p class="text-white fs-2 my-2">{asset.common_asset_data.name.clone()}</p>
                        </div>
                        <div class="col-md-auto">
                          if {burned} {
                              <i style="color: #ff0000; font-size: 1.75rem;" class="fa-solid fa-fire"></i>
                          }
                        </div>
                      </div>
                      <p class="text-white fs-5 mt-0">{format!("Power {}", asset.total_power)}</p>
                      <p class="text-white fs-4 mb-2">{format!("Set {}", asset.set.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Stage {}", asset.stage.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Wave {}", asset.wave.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Class {}", asset.class.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Affinity {}", asset.affinity.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Expression {}", asset.expression.clone())}</p>
                      <p class="text-white fs-4 mb-2"> {"Origin "}
                          <Link<Route> to={Route::Asset {token_address: asset.source_token_address.clone(), token_id: asset.source_disk_id}} classes="btn btn-primary me-1 mb-1">
                              { asset.source_disk_type.clone() }
                          </Link<Route>>
                      </p>
                      if {asset.origin_illuvitar_id.is_some()} {
                        <p class="text-white fs-4 mb-2">
                             {"Original Illuvitar "}
                             <Link<Route> to={Route::Asset {token_address: asset.common_asset_data.token_address.clone(),
                                                             token_id: asset.origin_illuvitar_id.unwrap()}} classes="btn btn-primary me-1 mb-1">
                                 { asset.common_asset_data.name.clone() }
                             </Link<Route>>
                        </p>
                      }
                      if {asset.accessorised_illuvitar_id.is_some()} {
                        <p class="text-white fs-4 mb-2">
                             {"Accessorised Illuvitar "}
                             <Link<Route> to={Route::Asset {token_address: asset.common_asset_data.token_address.clone(),
                                                             token_id: asset.accessorised_illuvitar_id.unwrap()}} classes="btn btn-primary me-1 mb-1">
                                 { asset.common_asset_data.name.clone() }
                             </Link<Route>>
                        </p>
                      }
                      if {!burned} {
                        <p class="text-white fs-4 mb-2">
                            {"Owned by "}
                            {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                        </p>
                      }
                      if {!asset.accessories.is_empty()} {
                          <div>
                            <p class="text-white fs-4 mb-2">{"Accessories"}</p>
                              { asset.accessories.iter().map(|accessory |
                              html!(
                                  <Link<Route> to={Route::Asset {token_address: accessory.token_address.clone(), token_id: accessory.token_id}} classes="btn btn-primary me-1 mb-1">
                                      { accessory.name.clone() }
                                  </Link<Route>>
                              )).collect::<Html>() }
                          </div>
                        }
                    </div>
                  </div>
                </div>
            </div>
        </div>
    )
}
