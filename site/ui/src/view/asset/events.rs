use crate::view::common::events_table::EventsTable;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
    pub token_id: i32,
}

#[function_component(AssetEvents)]
pub fn asset_events_function_component(props: &Props) -> Html {
    let url = format!(
        "/asset/events?token_address={}&token_id={}",
        &props.token_address, &props.token_id
    );

    return html! {
        <div class="container-fluid p-3 bg-dark">
            <EventsTable {url} />
        </div>
    };
}
