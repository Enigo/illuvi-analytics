use crate::utils::formatting_utils;
use crate::view::asset::order_data_view::AssetOrderData;
use crate::view::asset::{image::AssetImage, title::AssetTitle};
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
    let name = &asset.common_asset_data.name;
    let token_address = &asset.common_asset_data.token_address;
    let token_id = &asset.common_asset_data.token_id;
    let image_url = asset.common_asset_data.image_url.clone();
    let burned = false;
    let common_order_data = &asset.common_order_data;

    html!(
        <div class="container-fluid p-3 bg-gray">
            <div class="container animate__animated animate__fadeIn animate__faster">
                { html! { <AssetTitle name={name.clone()} {burned}/> } }
                <div class="row">
                  <div class="col-lg align-items-center justify-content-lg-start justify-content-center order-lg-2 text-center text-lg-start mt-3 p-0">
                    { html! { <AssetImage name={name.clone()} {image_url} {burned}/> } }
                    <div class="bg-dark p-3 rounded border border-2 border-dark my-3">
                      <p class="text-white fs-4 mb-2">{format!("Tier {}", asset.tier.clone())}</p>
                      if {asset.landmark != "None"} {
                          <p class="text-white fs-4 mb-2">{format!("Landmark {}", asset.landmark)}</p>
                      }
                    </div>
                  </div>
                  <div class="col-lg align-items-center order-lg-2 text-center text-lg-start ps-lg-4">
                    <div>
                      <p class="text-white fs-4 mb-2">
                          {"Owned by "}
                          {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                      </p>
                      { elements(&asset) }
                      { fuels(&asset) }
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
