use crate::view::common::transaction_view::SingleTransactionView;
use model::model::transaction::SingleTransaction;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewProps {
    pub token_address: String,
    pub transactions: Vec<SingleTransaction>,
}

#[function_component(TransactionsView)]
pub fn transactions_view(props: &ViewProps) -> Html {
    let token_address = &props.token_address;
    let transactions = &props.transactions;
    return transactions
        .iter()
        .enumerate()
        .map(|(index, trade)| {
            let trade = trade.clone();
            let token_address = token_address.clone();
            let render_border_end = index < &transactions.len() - 1;
            html!(
                <SingleTransactionView {token_address} {trade} {render_border_end}/>
            )
        })
        .collect::<Html>();
}
