use crate::view::wallet::events::WalletEvents;
use crate::view::wallet::wallet_data_view::WalletDataView;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub wallet: String,
}

#[function_component(Wallet)]
pub fn wallet_function_component(props: &Props) -> Html {
    html! {
         <selection>
            { html! {<WalletDataView wallet={props.wallet.clone()} />} }
            { html! {<WalletEvents wallet={props.wallet.clone()} />} }
         </selection>
    }
}
