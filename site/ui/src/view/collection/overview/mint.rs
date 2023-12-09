use crate::utils::{api_utils, pagination_utils};
use crate::view::loading::LoadingSpinnerDark;
use log::error;
use model::model::mint::MintData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionMint)]
pub fn collection_mint_function_component(props: &Props) -> Html {
    let page = use_state(|| 1);
    let mint = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let mint = mint.clone();
        let page = page.clone();
        let page_val = *page;
        use_effect_with((page_val, props.token_address.clone()), move |_| {
            mint.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match api_utils::fetch_single_api_response::<MintData>(
                    format!(
                        "/mint/mints?token_address={}&page={}",
                        token_address, page_val
                    )
                    .as_str(),
                )
                .await
                {
                    Ok(fetched_mint) => {
                        mint.set(Some(fetched_mint));
                    }
                    Err(e) => {
                        error!("{e}")
                    }
                }
            });
        });
    }

    return html! {
        <selection>
          <div class="container-fluid p-3 bg-dark">
             <div class="container mt-4 animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
                <p class="text-white text-center fs-2 mb-4">{"Explore"}</p>
                if let Some(mint) = (*mint).as_ref() {
                   { pagination_utils::pagination(mint.total, &page) }
                   { mint.mints.chunks(5).map(|mints| html! {
                       <div class="row justify-content-md-center g-0">
                           {
                               mints.iter().map(|mint| {
                                   html!
                                   {
                                       <div class="col-md mb-2 text-center">
                                           <Link<Route> to={Route::Asset {token_address: mint.token_address.to_string(), token_id: mint.token_id} } classes="img-fluid">
                                               <img src={mint.image_url.clone()} class="img-fluid" width="175" height="175"
                                               loading="lazy" alt={mint.name.clone()}/>
                                           </Link<Route>>
                                           <p class="text-white">{mint.name.clone()}</p>
                                       </div>
                                   }
                               }).collect::<Html>()
                           }
                       </div>
                   }).collect::<Html>() }
                } else {
                   <LoadingSpinnerDark />
                }
             </div>
           </div>
        </selection>
    };
}
