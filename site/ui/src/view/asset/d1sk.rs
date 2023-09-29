use crate::utils::formatting_utils;
use crate::view::asset::order_data_view::AssetOrderData;
use crate::view::asset::{image::AssetImage, title::AssetTitle};
use model::model::asset::D1skAssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub d1sk: D1skAssetData,
}

#[function_component(AssetD1sk)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = &props.d1sk;
    return html! {
        <section>
            {
                intro(&asset)
            }
        </section>
    };
}

fn intro(asset: &D1skAssetData) -> Html {
    let name = &asset.common_asset_data.name;
    let token_address = &asset.common_asset_data.token_address;
    let token_id = &asset.common_asset_data.token_id;
    let image_url = asset.common_asset_data.image_url.clone();
    let burned = asset.common_asset_data.burned.clone();
    let common_order_data = &asset.common_order_data;

    html!(
        <div class="container-fluid p-3 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                { html! { <AssetTitle name={name.clone()} {burned}/> } }

                <div class="row">
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start mt-3 p-0">
                    { html! { <AssetImage name={name.clone()} {image_url} {burned}/> } }
                    <div class="bg-dark p-3 rounded border border-2 border-dark my-3">
                      <p class="text-white fs-4 mb-2">{format!("Alpha {}", if {asset.alpha} { "Yes" } else {"No"})}</p>
                      <p class="text-white fs-4 mb-2">{format!("Set {}", asset.set.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Wave {}", asset.wave.clone())}</p>
                    </div>
                  </div>
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start ps-lg-4">
                    <div>
                      if {burned} {
                          <div>
                            <p class="text-white fs-4 mb-0">{"Content"}</p>
                            <div class="row text-center p-3 justify-content-left">
                                { asset.content.iter().map(|content|
                                html!(
                                    <div class="col-md-3 mb-2 mx-1 p-0 border border-muted rounded bg-dark d-flex flex-column">
                                        <p class="fs-5 text-white m-0 py-2">{&content.name}</p>
                                        <div class="d-flex justify-content-center align-items-end py-2 flex-grow-1">
                                          { formatting_utils::get_asset_link(&content.token_address, content.token_id, &content.image_url) }
                                        </div>
                                    </div>
                                )).collect::<Html>() }
                            </div>
                          </div>
                      } else {
                        <p class="text-white fs-4 mb-2">
                            {"Owned by "}
                            {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                        </p>
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
