use log::error;
use model::model::collection::CollectionData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::utils::api_utils;
use crate::view::search::Search;

#[function_component(Header)]
pub fn header() -> Html {
    let collections = use_state(|| vec![]);
    {
        let collections = collections.clone();
        use_effect_with((), move |_| {
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
        });
    }

    let collections = collections.iter().map(|collection| html! {
        <li class="nav-item dropdown">
          <a class="nav-link dropdown-toggle" href="#" id={format!("{}Link", collection.name.clone())} role="button" data-bs-toggle="dropdown" aria-expanded="false">
            { collection.name.clone() }
          </a>
          <ul class="dropdown-menu dropdown-menu-dark" aria-labelledby={format!("{}Link", collection.name.clone())}>
            <Link<Route> to={Route::Collection {token_address: collection.address.clone()} } classes="dropdown-item">
                { "Overview" }
            </Link<Route>>
            <Link<Route> to={Route::CollectionStats {token_address: collection.address.clone()} } classes="dropdown-item">
                { "Statistics" }
            </Link<Route>>
          </ul>
        </li>
    }).collect::<Html>();

    html! {
        <header class="bg-dark sticky-top">
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container justify-content-start">
                    <Link<Route> to={ Route::Home } classes="navbar-brand d-flex align-items-center">
                        <img src="/img/favicon.png" alt="IlluviAnalytics" width="35" height="35" class="d-inline-block align-text-top" />
                        { "IlluviAnalytics" }
                    </Link<Route>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent">
                      <span class="navbar-toggler-icon"></span>
                    </button>
                     <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav">
                            { collections }
                            <li class="nav-item">
                                <Link<Route> to={ Route::About } classes="nav-link">
                                    { "About" }
                                </Link<Route>>
                            </li>
                        </ul>
                    </div>
                    { html! { <Search /> } }
                </div>
            </nav>
        </header>
    }
}
