use yew::prelude::*;

#[function_component(NoData)]
pub fn no_data_component() -> Html {
    return html!(
        <div class="container-fluid p-3 bg-gray">
            <div class="container text-center">
                <p class="text-white fs-2 mb-2 animate__animated animate__headShake">{"Data is coming soon!"}</p>
            </div>
        </div>
    );
}
