use log::error;
use model::model::collection::CollectionData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::utils::api_utils;

#[function_component(Header)]
pub fn header() -> Html {
    let collections = use_state(|| vec![]);
    {
        let collections = collections.clone();
        use_effect_with_deps(
            move |_| {
                let collections = collections.clone();
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
            <li class="nav-item">
                <Link<Route> to={Route::Collection {token_address: collection.address.clone()} } classes="nav-link">
                    { collection.name.clone() }
                </Link<Route>>
            </li>
        }).collect::<Html>();

    html! {
        <header class="bg-dark sticky-top">
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                    <Link<Route> to={ Route::Home } classes="navbar-brand">
                        { "IlluviAnalytics" }
                    </Link<Route>>
                    <div class="container-fluid justify-content-start">
                        <ul class="navbar-nav">
                            {
                                collections
                            }
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
