use log::info;
use route::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::view::{footer::Footer, header::Header};

mod route;
mod utils;
mod view;

#[function_component(App)]
fn app() -> Html {
    html! {
         <BrowserRouter>
            <Header />
            <Switch<Route> render={switch} />
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting app...");
    yew::Renderer::<App>::new().render();
}
