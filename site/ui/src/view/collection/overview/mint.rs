use crate::utils::api_utils;
use crate::view::loading::LoadingSpinnerDark;
use log::error;
use model::model::mint::MintData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

const MINTS_PER_PAGE: i8 = 50;

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
        use_effect_with_deps(
            move |_| {
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
            },
            (page_val, props.token_address.clone()),
        );
    }
    return match (*mint).as_ref() {
        Some(mint) => {
            html! {
                { mints(mint, page) }
            }
        }
        None => {
            html! {
                <LoadingSpinnerDark />
            }
        }
    };
}

pub fn mints(mint: &MintData, page: UseStateHandle<i32>) -> Html {
    let mints = mint.mints.chunks(5).map(|mints| html! {
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
    }).collect::<Html>();

    return html! {
    if !mint.mints.is_empty() {
            <selection>
                <div class="container-fluid p-3 bg-dark">
                    <div class="container mt-4 animate__animated animate__fadeIn animate__faster animate__delay-0.25s">
                        <p class="text-white text-center fs-2 mb-4">{"Explore"}</p>
                        {pagination(mint, page)}
                        {mints}
                    </div>
                </div>
            </selection>
        }
    };
}

fn pagination(mint: &MintData, page: UseStateHandle<i32>) -> Html {
    fn get_onclick(page: &UseStateHandle<i32>) -> Callback<i32> {
        let page_count_onclick = {
            let page = page.clone();
            Callback::from(move |count| {
                page.set(count);
            })
        };
        page_count_onclick
    }

    fn get_single_li_with_arrow(
        page: &UseStateHandle<i32>,
        page_value: i32,
        disabled: bool,
        arrow_class: &str,
    ) -> Html {
        let page_count_onclick = get_onclick(page);
        let mut class = String::from("page-link text-white shadow-none");
        if disabled {
            class.push_str(" bg-secondary disabled");
        } else {
            class.push_str(" bg-dark")
        }

        html!(
            <li class="page-item"><button onclick={move |_| page_count_onclick.emit(page_value)}
                                    class={ class }><i class={String::from(arrow_class)}></i></button></li>
        )
    }

    fn get_single_li_with_number(page: &UseStateHandle<i32>, page_value: i32) -> Html {
        let current_page = *page.clone();
        let page_count_onclick = get_onclick(page);
        let mut class = String::from("page-link bg-dark text-white shadow-none");
        if current_page == page_value {
            class.push_str(" active");
        }

        html!(
            <li class="page-item"><button onclick={move |_| page_count_onclick.emit(page_value)}
                                    class={ class }>{ page_value }</button></li>
        )
    }

    fn get_page_numbers(current_page: i32, max_page: i32) -> Vec<i32> {
        let mut start_page = std::cmp::max(1, current_page.saturating_sub(2));
        let end_page = std::cmp::min(start_page + 4, max_page);
        let mut num_pages = end_page - start_page + 1;

        if num_pages < 5 {
            start_page = max_page - 4;
            num_pages = 5;
        }

        (0..num_pages).map(|i| start_page + i).collect()
    }

    let total_page_count = (mint.total / MINTS_PER_PAGE as i64) as i32;

    let pages = get_page_numbers(*page, total_page_count)
        .iter()
        .map(|page_value| {
            html! {
                { get_single_li_with_number(&page, *page_value) }
            }
        })
        .collect::<Html>();

    html! {
        <div class="row">
          <div class="d-flex justify-content-center mt-3">
            <nav>
              <ul class="pagination">
                { get_single_li_with_arrow(&page, 1, *page == 1, "fas fa-angle-double-left")}
                { get_single_li_with_arrow(&page, *page - 1, *page == 1, "fas fa-angle-left")}
                { pages }
                { get_single_li_with_arrow(&page, *page + 1, *page == total_page_count, "fas fa-angle-right")}
                { get_single_li_with_arrow(&page, total_page_count, *page == total_page_count, "fas fa-angle-double-right")}
              </ul>
            </nav>
          </div>
        </div>
    }
}
