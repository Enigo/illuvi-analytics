use crate::utils::formatting_utils;
use crate::view::asset::{image::AssetImage, title::AssetTitle};
use model::model::asset::IlluvitarAssetData;
use yew::prelude::*;

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
    let name = &asset.common_asset_data.name;
    let image_url = asset.common_asset_data.image_url.clone();
    let burned = asset.common_asset_data.burned.clone();
    let accessorised_illuvitar = &asset.accessorised_illuvitar;
    let origin_illuvitar = &asset.origin_illuvitar;
    let d1sk = &asset.d1sk;
    html! {
        <div class="container-fluid p-3 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                { html! { <AssetTitle name={name.clone()} {burned}/> } }
                <div class="row">
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start mt-3">
                      { html! { <AssetImage name={name.clone()} {image_url} {burned}/> } }
                      <div class="bg-dark p-3 rounded border border-2 border-dark my-3">
                        <p class="text-white fs-5 mt-0">{format!("Power {}", asset.total_power)}</p>
                        <p class="text-white fs-4 mb-2">{format!("Set {}", asset.set.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Stage {}", asset.stage.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Wave {}", asset.wave.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Class {}", asset.class.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Affinity {}", asset.affinity.clone())}</p>
                        <p class="text-white fs-4 mb-2">{format!("Expression {}", asset.expression.clone())}</p>
                      </div>
                  </div>
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start ps-lg-4">
                    <div>
                      if {!burned} {
                        <p class="text-white fs-4 mb-2">
                            {"Owned by "}
                            {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                        </p>
                      }
                      if let Some(d1sk) = d1sk {
                        <div>
                            <p class="text-white fs-4 mb-0">{"Origin"}</p>
                            <div class="row text-center p-3 justify-content-left">
                                <div class="col-md-3 text-center mb-2 mx-1 p-0 border border-muted rounded bg-dark">
                                   <p class="fs-5 text-white m-0 p-1">{&d1sk.name}</p>
                                   <div class="justify-content-center align-items-end py-2">
                                     { formatting_utils::get_asset_link(&d1sk.token_address, d1sk.token_id, &d1sk.image_url) }
                                   </div>
                                </div>
                            </div>
                        </div>
                      }
                      if let Some(origin_illuvitar) = origin_illuvitar {
                        <div>
                            <p class="text-white fs-4 mb-0">{"Original Illuvitar"}</p>
                            <div class="row text-center p-3 justify-content-left">
                                <div class="col-md-3 text-center mb-2 mx-1 p-0 border border-muted rounded bg-dark">
                                   <p class="fs-5 text-white m-0 p-1">{&origin_illuvitar.name}</p>
                                   <div class="justify-content-center align-items-end py-2">
                                     { formatting_utils::get_asset_link(&origin_illuvitar.token_address, origin_illuvitar.token_id, &origin_illuvitar.image_url) }
                                   </div>
                                </div>
                            </div>
                        </div>
                      }
                      if let Some(accessorised_illuvitar) = accessorised_illuvitar {
                        <div>
                            <p class="text-white fs-4 mb-0">{"Accessorised Illuvitar"}</p>
                            <div class="row text-center p-3 justify-content-left">
                                <div class="col-md-3 text-center mb-2 mx-1 p-0 border border-muted rounded bg-dark">
                                   <p class="fs-5 text-white m-0 p-1">{&accessorised_illuvitar.name}</p>
                                   <div class="justify-content-center align-items-end py-2">
                                     { formatting_utils::get_asset_link(&accessorised_illuvitar.token_address, accessorised_illuvitar.token_id, &accessorised_illuvitar.image_url) }
                                   </div>
                                </div>
                            </div>
                        </div>
                      }
                      if {!asset.accessories.is_empty()} {
                          <div>
                             <p class="text-white fs-4 mb-0">{"Accessories"}</p>
                             <div class="row text-center p-3 justify-content-left">
                                  { asset.accessories.iter().map(|accessory |
                                  html!(
                                    <div class="col-md-3 mb-2 mx-1 p-0 border border-muted rounded bg-dark d-flex flex-column">
                                        <p class="fs-5 text-white m-0 p-1">{&accessory.name}</p>
                                        <div class="d-flex justify-content-center align-items-end py-2 flex-grow-1">
                                          { formatting_utils::get_asset_link(&accessory.token_address, accessory.token_id, &accessory.image_url) }
                                        </div>
                                    </div>
                                  )).collect::<Html>() }
                             </div>
                          </div>
                        }
                    </div>
                  </div>
                </div>
            </div>
        </div>
    }
}
