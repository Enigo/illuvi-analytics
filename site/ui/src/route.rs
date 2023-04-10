use crate::view::asset::AssetLand;
use crate::view::home::Home;
use crate::view::collection::Collection;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:token_address")]
    Collection { token_address: String },
    #[at("/:token_address/:token_id")]
    Asset {
        token_address: String,
        token_id: i32,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::Collection { token_address } => html! {<Collection token_address={token_address}/>},
        Route::Asset {
            token_address,
            token_id,
        } => html! {<AssetLand token_address={token_address} token_id={token_id}/>},
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}
