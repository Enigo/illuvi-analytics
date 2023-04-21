use crate::utils::api_utils;
use log::error;
use model::model::mint::MintData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

const LAND_ICON: &str = "https://assets.illuvium-game.io/illuvidex/land/land-";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(Collection)]
pub fn collection_function_component(props: &Props) -> Html {
    let mints = use_state(|| vec![]);
    {
        let token_address = props.token_address.clone();
        let mints = mints.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<Vec<MintData>>(
                        format!("/mint/mints?token_address={}", token_address.clone()).as_str(),
                    )
                    .await
                    {
                        Ok(fetched_mints) => {
                            mints.set(fetched_mints);
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
            props.token_address.clone(),
        );
    }

    // take(100) should be removed - replaced with pagination
    let mints: Vec<&MintData> = mints.iter().take(100).collect();
    let mints = mints.chunks(4).map(|mints| html! {
        <div class="row justify-content-md-center">
            {
                mints.iter().map(|mint| {
                    html!
                    {
                        <div class="col text-center">
                            <Link<Route> to={Route::Asset {token_address: mint.token_address.to_string(), token_id: mint.token_id} } classes="img-fluid">
                                <img src={format!("{}{}{}", LAND_ICON, mint.token_id, ".svg")}
                                class="img-fluid"
                                loading="lazy" alt={mint.name.clone()}/>
                            </Link<Route>>
                            <p class="text-white">{mint.name.clone()}</p>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
        }).collect::<Html>();

    return html! {
        <div class="container mt-4">
            {mints}
        </div>
    };
}
