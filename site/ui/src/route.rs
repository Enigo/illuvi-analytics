use crate::view::{
    about::About, asset::page::Asset, collection::overview::page::Collection,
    collection::stats::page::CollectionStats, home::Home, wallet::page::Wallet,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/:token_address/:token_id")]
    Asset {
        token_address: String,
        token_id: i32,
    },
    #[at("/:token_address/overview")]
    Collection { token_address: String },
    #[at("/:token_address/stats")]
    CollectionStats { token_address: String },
    #[at("/wallet/:wallet")]
    Wallet { wallet: String },
    #[at("/about")]
    About,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::About => html! { <About />},
        Route::Wallet { wallet } => html! { <Wallet wallet={wallet}/>},
        Route::Collection { token_address } => html! {<Collection token_address={token_address}/>},
        Route::CollectionStats { token_address } => {
            html! {<CollectionStats token_address={token_address}/>}
        }
        Route::Asset {
            token_address,
            token_id,
        } => html! {<Asset token_address={token_address} token_id={token_id}/>},
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}
