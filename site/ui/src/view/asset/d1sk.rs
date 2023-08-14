use crate::route::Route;
use crate::utils::formatting_utils;
use model::model::asset::D1skAssetData;
use yew::prelude::*;
use yew_router::prelude::*;

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
    let burned = asset.common_asset_data.burned.clone();

    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
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
                      <p class="text-white fs-4 mb-2">{format!("Alpha {}", if {asset.alpha} { "Yes" } else {"No"})}</p>
                      <p class="text-white fs-4 mb-2">{format!("Set {}", asset.set.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Wave {}", asset.wave.clone())}</p>
                      if {burned} {
                          <div>
                            <p class="text-white fs-4 mb-2">{"Content"}</p>
                              { asset.content.iter().map(|content|
                              html!(
                                  <Link<Route> to={Route::Asset {token_address: content.token_address.clone(), token_id: content.token_id}} classes="btn btn-primary me-1 mb-1">
                                      { content.name.clone() }
                                  </Link<Route>>
                              )).collect::<Html>() }
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
            </div>
        </div>
    )
}
