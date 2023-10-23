use yew::prelude::*;

const ITEMS_PER_PAGE: i64 = 50;

pub fn pagination(total: i64, page: &UseStateHandle<i32>) -> Html {
    let total = if total < ITEMS_PER_PAGE {
        ITEMS_PER_PAGE
    } else {
        total
    };
    let total_page_count = (total as f64 / ITEMS_PER_PAGE as f64).ceil() as i32;

    let current_page = *page.clone();
    let pages = get_page_numbers(current_page, total_page_count)
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
                { get_single_li_with_arrow(&page, 1, current_page == 1, "fas fa-angle-double-left")}
                { get_single_li_with_arrow(&page, current_page - 1, current_page == 1, "fas fa-angle-left")}
                { pages }
                { get_single_li_with_arrow(&page, current_page + 1, current_page == total_page_count, "fas fa-angle-right")}
                { get_single_li_with_arrow(&page, total_page_count, current_page == total_page_count, "fas fa-angle-double-right")}
              </ul>
            </nav>
          </div>
        </div>
    }
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
    if max_page <= 5 {
        return (1..=max_page).collect();
    }

    let mut start_page = std::cmp::max(1, current_page - 2);
    let mut end_page = std::cmp::min(current_page + 2, max_page);

    while (end_page - start_page + 1) < 5 {
        if start_page > 1 {
            start_page -= 1;
        }
        if end_page < max_page {
            end_page += 1;
        }
    }

    (start_page..=end_page).collect()
}

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
