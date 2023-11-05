use crate::utils::{api_utils, navigation_utils};
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
        use_effect_with(
            (), move |_| {
                navigation_utils::scroll_to_top();
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
        );
    }

    let collections = collections.iter().map(|collection| html! {
            <div class="col-md-auto col-6 text-center animate__animated animate__fadeInUp animate__slow animate__delay-0.75s">
                <Link<Route> to={Route::Collection {token_address: collection.address.clone()} }>
                    <img src={collection.collection_image_url.clone()} class="img-responsive" alt={collection.name.clone()}/>
                </Link<Route>>
                <p class="text-white">{collection.name.clone()}</p>
            </div>
        }).collect::<Html>();

    return html! {
        <div class="container-fluid bg-gray vh-100">
            <div class="container">
                 <div class="row justify-content-center align-items-center my-3">
                     <div class="col text-center">
                        <p class="text-white text-center fs-1 fw-bold my-5 animate__animated animate__zoomInDown animate__delay-0.5s">{"Explore the Data of Illuvium Universe!"} </p>
                     </div>
                 </div>
                 <div class="row justify-content-md-center">
                    { collections }
                </div>
            </div>
        </div>
    };
}
