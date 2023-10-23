use crate::view::common::events_table::EventsTable;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub wallet: String,
}

#[function_component(WalletEvents)]
pub fn wallet_events_function_component(props: &Props) -> Html {
    let url = format!("/wallet/events?wallet={}", props.wallet);

    return html! {
        <div class="container-fluid p-3 bg-dark">
            <EventsTable {url} />
        </div>
    };
}
