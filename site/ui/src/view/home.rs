use log::error;
use crate::utils::api_utils;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function_component() -> Html {
    let token_addresses = use_state(|| vec![]);
    {
        let token_addresses = token_addresses.clone();
        use_effect_with_deps(
            move |_| {
                let token_addresses = token_addresses.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<Vec<String>>(
                        "/mint/token_addresses",
                    )
                    .await
                    {
                        Ok(fetched_token_addresses) => {
                            token_addresses.set(fetched_token_addresses);
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let token_addresses = token_addresses.iter().map(|token_address| html! {
            <div class="col">
                <a href={token_address.clone()}>
                    { token_address }
                </a>
            </div>
        }).collect::<Html>();

    return html! {
        <div class="grid-container">
            <div class="container mt-4">
                 <div class="row justify-content-md-center">
                    {token_addresses}
                </div>
            </div>
        </div>
    };
}
