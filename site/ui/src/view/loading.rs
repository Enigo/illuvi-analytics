use yew::prelude::*;

#[function_component(LoadingSpinnerGray)]
pub fn spinner_gray() -> Html {
    return spinner(String::from("bg-gray"));
}

#[function_component(LoadingSpinnerDark)]
pub fn spinner_dark() -> Html {
    return spinner(String::from("bg-dark"));
}

fn spinner(background: String) -> Html {
    return html!(
        <selection>
            <div class={format!("container-fluid pt-5 {}", background)}>
                <div class="container">
                  <div class="spinner-container">
                    <div class="spinner"></div>
                  </div>
                </div>
            </div>
        </selection>
    );
}
