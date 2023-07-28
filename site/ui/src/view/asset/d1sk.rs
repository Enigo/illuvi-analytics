use crate::utils::formatting_utils;
use model::model::asset::D1skAssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub d1sk: D1skAssetData,
}

#[function_component(AssetD1sk)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let d1sk = &props.d1sk;
    return html! {
        <section>
            {
                intro(&d1sk)
            }
        </section>
    };
}

fn intro(asset: &D1skAssetData) -> Html {
    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__fast">
                <div class="row g-0">
                  <div class="col-lg-5 order-lg-1 d-flex align-items-center justify-content-center text-center">
                    <img src={asset.common_asset_data.image_url.clone()}
                      class="w-75 img-fluid shadow-gradient"
                      loading="lazy" alt={asset.common_asset_data.name.clone()}/>
                  </div>
                  <div class="col-lg-7 d-flex align-items-center order-lg-2 text-center text-lg-start p-md-5">
                    <div class="d-flex flex-column">
                      <p class="text-white fs-2 my-2">{asset.common_asset_data.name.clone()}</p>
                      <p class="text-white fs-4 mb-2">{format!("Alpha {}", if {asset.alpha} { "Yes" } else {"No"})}</p>
                      <p class="text-white fs-4 mb-2">{format!("Set {}", asset.set.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Wave {}", asset.wave.clone())}</p>
                      <p class="text-white fs-4 mb-2">
                          if {asset.common_asset_data.burned} {
                            {"Burnt"}
                          } else {
                            {"Owned by "}
                            {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                          }
                      </p>
                    </div>
                  </div>
                </div>
            </div>
        </div>
    )
}
