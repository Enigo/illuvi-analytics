use crate::utils::{api_utils, formatting_utils};
use crate::view::loading::LoadingSpinnerGrayNoVh;
use gloo_timers::callback::Timeout;
use log::error;
use model::model::search::SearchData;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    let search = use_state(|| String::new());
    let search_data = use_state(|| None);
    let typed_state = use_state(|| false);
    {
        let search = search.clone();
        let search_data = search_data.clone();
        use_effect_with((search.clone(), *typed_state), move |_| {
            if !search.is_empty() {
                let search = search.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<SearchData>(
                        format!("/search?search={}", search.as_str()).as_str(),
                    )
                    .await
                    {
                        Ok(fetched_search) => {
                            search_data.set(Some(fetched_search));
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            }
        });
    }

    let timeout_state = use_state(|| None);
    let typed_state_clone = typed_state.clone();
    let search_data_clone = search_data.clone();
    let search_clone = search.clone();
    let oninput = Callback::from(move |e: InputEvent| {
        if let Some(target) = e.target() {
            let input = target.dyn_into::<HtmlInputElement>().ok();
            if let Some(input) = input {
                let value = input.value();
                if value.is_empty() {
                    search_data_clone.set(None);
                    typed_state_clone.set(false);
                } else {
                    typed_state_clone.set(true);
                    search_data_clone.set(None);
                    let prev_timeout = timeout_state.clone();
                    if prev_timeout.is_some() {
                        drop(prev_timeout);
                    }
                    let search = search_clone.clone();
                    let timeout = Timeout::new(1_000, move || {
                        search.set(value);
                    });
                    timeout_state.set(Some(timeout));
                }
            }
        }
    });

    let onsubmit = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
    });

    let typed_state_clone = typed_state.clone();
    let onfocusin = Callback::from(move |e: FocusEvent| {
        if let Some(target) = e.target() {
            let input = target.dyn_into::<HtmlInputElement>().ok();
            if let Some(input) = input {
                let value = input.value();
                if !value.is_empty() {
                    typed_state_clone.set(true);
                    search.set(value);
                }
            }
        }
    });

    let typed_state_clone = typed_state.clone();
    let search_data_clone = search_data.clone();
    let onfocusout = Callback::from(move |_| {
        let typed_state_clone = typed_state_clone.clone();
        let search_data_clone = search_data_clone.clone();
        Timeout::new(150, move || {
            search_data_clone.set(None);
            typed_state_clone.set(false);
        })
        .forget();
    });

    let typed_state_clone = typed_state.clone();
    let search_data_clone = search_data.clone();
    let onclick = Callback::from(move |_| {
        search_data_clone.set(None);
        typed_state_clone.set(false);
    });

    let search_result_html = match (*search_data).as_ref() {
        Some(search_data) => {
            if search_data.asset_content_data.is_empty() {
                html! {<li><p class="dropdown-item text-white my-2">{"No results found..."}</p></li>}
            } else {
                search_data.asset_content_data.iter().map(|asset| {
                    let onclick = onclick.clone();
                    html!(
                        <li>
                            <div class="align-items-center text-center text-white m-3" {onclick}>
                                {formatting_utils::get_asset_link(&asset.token_address, asset.token_id, &asset.image_url)}
                                <p class="m-0">{asset.name.clone()}</p>
                            </div>
                        </li>
                    )
                }).collect::<Html>()
            }
        }
        None => {
            html! {
                <LoadingSpinnerGrayNoVh />
            }
        }
    };

    html! {
      <div class="row">
        <div class="col-md-8">
          <form class="input-group bg-dark border border-white rounded" {onsubmit} {onfocusin} {onfocusout}>
            <input id="search" type="search" autocomplete="off" class="form-control bg-dark border-0 text-white shadow-none" placeholder="Token Id or Name" aria-label="Search"
                {oninput}/>
            <span class="input-group-text bg-dark border-0 rounded text-white"><i class="fas fa-search"></i></span>
            <div class="w-100">
              <ul class={format!("dropdown-menu bg-gray border border-top-0 border-1 border-white w-100 max-vh-90 p-0 overflow-auto {}", if *typed_state.clone() {"show"} else {"hide"})}>
                { search_result_html }
              </ul>
            </div>
          </form>
        </div>
      </div>
    }
}
