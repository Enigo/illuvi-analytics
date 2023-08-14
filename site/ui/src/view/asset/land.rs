use crate::utils::formatting_utils;
use model::model::asset::LandAssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub land: LandAssetData,
}

#[function_component(AssetLand)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = &props.land;
    return html! {
        <section>
            {
                intro(&asset)
            }
        </section>
    };
}

fn intro(asset: &LandAssetData) -> Html {
    html!(
        <div class="container-fluid p-5 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                <div class="row g-0">
                  <div class="col-lg-5 order-lg-1 d-flex align-items-center justify-content-center text-center">
                    <img src={asset.common_asset_data.image_url.clone()}
                      class="w-75 img-fluid shadow-gradient"
                      loading="lazy" alt={asset.common_asset_data.name.clone()}/>
                  </div>
                  <div class="col-lg-7 d-flex align-items-center order-lg-2 text-center text-lg-start p-md-5">
                    <div class="d-flex flex-column">
                      <p class="text-white fs-2 my-2">{asset.common_asset_data.name.clone()}</p>
                      <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                      if {asset.landmark != "None"} {
                          <p class="text-white fs-4 mb-2">{format!("Landmark {}", asset.landmark)}</p>
                      }
                      <p class="text-white fs-4 mb-2">
                          {"Owned by "}
                          {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                      </p>
                      { elements(&asset) }
                      { fuels(&asset) }
                    </div>
                  </div>
                </div>
            </div>
        </div>
    )
}

fn elements(asset: &LandAssetData) -> Html {
    html!(
        <div class="row mb-2">
            <p class="text-white fs-4 m-0">{"Elements"}</p>
            <div class="col-md-4 d-flex align-items-center justify-content-center">
                <img src="/img/carbon.png" class="img-fluid mb-0 shadow-gradient" alt="Carbon" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Carbon x{}", asset.carbon.clone())}</p>
            </div>
            <div class="col-md-4 d-flex align-items-center justify-content-center px-0">
                <img src="/img/hydrogen.png" class="img-fluid mb-0 shadow-gradient" alt="Hydrogen" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Hydrogen x{}", asset.hydrogen.clone())}</p>
            </div>
            <div class="col-md-4 d-flex align-items-center justify-content-center px-0">
                <img src="/img/silicon.png" class="img-fluid mb-0 shadow-gradient" alt="Silicon" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Silicon x{}", asset.silicon.clone())}</p>
            </div>
        </div>
    )
}

fn fuels(asset: &LandAssetData) -> Html {
    html!(
        <div class="row mb-0">
            <p class="text-white fs-4 m-0">{"Fuels"}</p>
            <div class="col-md-4 d-flex align-items-center justify-content-center">
                <img src="/img/crypton.png" class="img-fluid mb-0 shadow-gradient" alt="Crypton" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Crypton x{}", asset.crypton.clone())}</p>
            </div>
            <div class="col-md-4 d-flex align-items-center justify-content-center px-0">
                <img src="/img/hyperion.png" class="img-fluid mb-0 shadow-gradient" alt="Hyperion" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Hyperion x{}", asset.hyperion.clone())}</p>
            </div>
            <div class="col-md-4 d-flex align-items-center justify-content-center px-0">
                <img src="/img/solon.png" class="img-fluid mb-0 shadow-gradient" alt="Solon" width="25%"/>
                <p class="text-white fs-5 mb-0 ps-1">{format!("Solon x{}", asset.solon.clone())}</p>
            </div>
        </div>
    )
}
