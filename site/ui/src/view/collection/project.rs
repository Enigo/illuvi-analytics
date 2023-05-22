use crate::utils::api_utils;
use log::error;
use model::model::collection::CollectionData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionProject)]
pub fn collection_project_function_component(props: &Props) -> Html {
    let collection = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let collection = collection.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<CollectionData>(
                        format!("/collection/collection?token_address={}", token_address).as_str(),
                    )
                    .await
                    {
                        Ok(fetched_data) => {
                            collection.set(Some(fetched_data));
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

    return match (*collection).as_ref() {
        Some(data) => project(data),
        None => {
            html! {
                <div class="container pt-5">
                    <p class="text-white fs-4 mb-2">{"Loading..."}</p>
                </div>
            }
        }
    };
}

fn project(collection: &CollectionData) -> Html {
    html! {
        <div class="container mt-1 p-3 bg-dark">
            <div class="row mt-4">
                <div class="col-lg-4 text-center">
                  <img src={collection.collection_image_url.clone()} class="img-fluid" width="250" height="250" alt={collection.name.clone()}/>
                </div>
                <div class="col-lg-7">
                  <h2 class="text-white fs-1">{&collection.name}</h2>
                  <p class="text-light">{&collection.description}</p>
                </div>
            </div>
        </div>
    }
}
