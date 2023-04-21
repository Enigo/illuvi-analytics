use crate::utils::api_utils;
use log::error;
use model::model::collection::CollectionData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Home)]
pub fn home_function_component() -> Html {
    let collections = use_state(|| vec![]);
    {
        let collections = collections.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<Vec<CollectionData>>(
                        "/collection/collections",
                    )
                    .await
                    {
                        Ok(fetched_collections) => {
                            collections.set(fetched_collections);
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
            (),
        );
    }

    let collections = collections.iter().map(|collection| html! {
            <div class="col text-center">
                <Link<Route> to={Route::Collection {token_address: collection.address.clone()} } classes="img-fluid">
                    <img src={collection.collection_image_url.clone()} class="img-fluid" width="250" height="250" alt={collection.name.clone()}/>
                </Link<Route>>
                <p class="text-white">{collection.name.clone()}</p>
            </div>
        }).collect::<Html>();

    return html! {
        <div class="container mt-4">
             <div class="row justify-content-md-center">
                { collections }
            </div>
        </div>
    };
}
