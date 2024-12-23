use chrono::Datelike;
use chrono::Utc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-dark text-muted border-top mt-auto">
          <div class="container py-2">
            <div class="row">
              <div class="col-12 d-flex justify-content-between align-items-center text-center">
                <p class="fs-6 mb-1">{format!("© 2023-{} Developed with 💜 for ", get_current_year())}<a href="https://illuvium.io" target="_blank" class="text-decoration-none">{ "Illuvium" }</a>{" community and crypto enthusiasts"}</p>
                <p class="ms-auto"><a href="https://github.com/Enigo/illuvi-analytics" target="_blank"><i class="fab fa-github fa-2x"></i></a></p>
              </div>
            </div>
            <div class="row">
              <div class="d-flex justify-content-center justify-content-md-start">
                <Link<Route> to={ Route::About } classes="nav-link">
                    { "About" }
                </Link<Route>>
              </div>
            </div>
          </div>
        </footer>
    }
}

fn get_current_year() -> i32 {
    let now = Utc::now();
    let year = now.year();
    return year;
}
