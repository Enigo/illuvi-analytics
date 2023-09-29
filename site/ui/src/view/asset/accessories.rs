use crate::utils::formatting_utils;
use crate::view::asset::order_data_view::AssetOrderData;
use crate::view::asset::{image::AssetImage, title::AssetTitle};
use model::model::asset::AccessoriesAssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub accessories: AccessoriesAssetData,
}

#[function_component(AssetAccessories)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = &props.accessories;
    return html! {
        <section>
            {
                intro(&asset)
            }
        </section>
    };
}

fn intro(asset: &AccessoriesAssetData) -> Html {
    let name = &asset.common_asset_data.name;
    let token_address = &asset.common_asset_data.token_address;
    let token_id = &asset.common_asset_data.token_id;
    let image_url = asset.common_asset_data.image_url.clone();
    let burned = asset.common_asset_data.burned.clone();
    let illuvitar = asset.illuvitar.clone();
    let d1sk = &asset.d1sk;
    let common_order_data = &asset.common_order_data;

    html!(
        <div class="container-fluid p-3 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                { html! { <AssetTitle name={name.clone()} {burned}/> } }
                <div class="row">
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start mt-3 p-0">
                    { html! { <AssetImage name={name.clone()} {image_url} {burned}/> } }
                    <div class="bg-dark p-3 rounded border border-2 border-dark my-3">
                      <p class="text-white fs-5 mt-0">{format!("x{}% boost", asset.multiplier)}</p>
                      <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Stage {}", asset.stage.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Slot {}", asset.slot.clone())}</p>
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
                      if let Some(illuvitar) = illuvitar {
                        <div>
                          <p class="text-white fs-4 mb-0">{"Bound to "}</p>
                          <div class="row text-center p-3 justify-content-left">
                              <div class="col-md-3 text-center mb-2 mx-1 p-0 border border-muted rounded bg-dark">
                                  <p class="fs-5 text-white m-0 p-1">{&illuvitar.name}</p>
                                  <div class="justify-content-center align-items-end py-2">
                                    { formatting_utils::get_asset_link(&illuvitar.token_address, illuvitar.token_id, &illuvitar.image_url) }
                                  </div>
                              </div>
                          </div>
                        </div>
                      }
                    </div>
                  </div>
                </div>

                if let Some(common_order_data) = common_order_data {
                    <AssetOrderData common_order_data={common_order_data.clone()} token_address={token_address.clone()} {token_id}/>
                }

            </div>
        </div>
    )
}
