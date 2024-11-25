use crate::utils::formatting_utils;
use crate::view::asset::order_data_view::AssetOrderData;
use crate::view::asset::{image::AssetImage, title::AssetTitle};
use model::model::asset::EventAssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub event: EventAssetData,
}

// not to confuse with Events!
// this one is NFT, and events are operations perfromed with the given asset
#[function_component(AssetEvent)]
pub fn asset_land_function_component(props: &Props) -> Html {
    let asset = &props.event;
    return html! {
        <section>
            {
                intro(&asset)
            }
        </section>
    };
}

fn intro(asset: &EventAssetData) -> Html {
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
                      <p class="text-white fs-4 mb-2">{format!("Line {}", asset.line.clone())}</p>
                      <p class="text-white fs-4 mb-2">{format!("Promotion {}", asset.promotion.clone())}</p>
                    </div>
                  </div>
                  <div class="col-lg align-items-center order-lg-2 text-center text-lg-start ps-lg-4">
                    if !burned {
                        <div>
                          <p class="text-white fs-4 mb-2">
                              {"Owned by "}
                              {formatting_utils::format_wallet_link(&asset.common_asset_data.current_owner)}
                          </p>
                        </div>
                    }
                  </div>
                </div>

                if let Some(common_order_data) = common_order_data {
                    <AssetOrderData common_order_data={common_order_data.clone()} token_address={token_address.clone()} {token_id}/>
                }

            </div>
        </div>
    )
}
