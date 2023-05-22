use crate::view::collection::project::CollectionProject;
use crate::view::collection::stats::stats_view::CollectionStatsView;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
}

#[function_component(CollectionStats)]
pub fn collection_stats_function_component(props: &Props) -> Html {
    html!(
        <selection>
            { html! {<CollectionProject token_address={props.token_address.clone()} />} }
            { html! {<CollectionStatsView token_address={props.token_address.clone()} />} }
        </selection>
    )
}
