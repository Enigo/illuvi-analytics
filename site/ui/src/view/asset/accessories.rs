use crate::route::Route;
use crate::utils::formatting_utils;
use model::model::asset::AccessoriesAssetData;
use yew::prelude::*;
use yew_router::prelude::*;

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
    let burned = asset.common_asset_data.burned.clone();
    let illuvitar = asset.illuvitar.clone();
    let illuvitar_html = if illuvitar.is_some() {
        let illuvitar = illuvitar.unwrap();
        html!(
           <p class="text-white fs-4 mb-2">
                {"Bound to "}
                <Link<Route> to={Route::Asset {token_address: illuvitar.token_address.clone(),
                                                token_id: illuvitar.token_id}} classes="btn btn-primary me-1 mb-1">
                    { illuvitar.name.clone() }
                </Link<Route>>
            </p>
        )
    } else {
        html!()
    };

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
                      <p class="text-white fs-5 mt-0">{format!("x{}% boost", asset.multiplier)}</p>
                      <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Stage {}", asset.stage.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Slot {}", asset.slot.clone())}</p>
                      <p class="text-white fs-4 mb-2"> {"Origin "}
                      <Link<Route> to={Route::Asset {token_address: asset.source_token_address.clone(), token_id: asset.source_disk_id}} classes="btn btn-primary me-1 mb-1">
                          { asset.source_disk_type.clone() }
                      </Link<Route>>
                      </p>
                      if {burned} {
                        { illuvitar_html }
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
