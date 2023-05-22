use yew::prelude::*;

use crate::view::collection::overview::{mint::CollectionMint, vitals::CollectionVitals};
use crate::view::collection::project::CollectionProject;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(Collection)]
pub fn collection_function_component(props: &Props) -> Html {
    html! {
         <selection>
            { html! {<CollectionProject token_address={props.token_address.clone()} />} }
            { html! {<CollectionVitals token_address={props.token_address.clone()} />} }
            { html! {<CollectionMint token_address={props.token_address.clone()} />} }
         </selection>
    }
}
