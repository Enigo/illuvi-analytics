use yew::prelude::*;

#[function_component(LoadingSpinnerGray)]
pub fn spinner_gray() -> Html {
    return spinner(String::from("bg-gray"), String::from("vh-100"));
}

#[function_component(LoadingSpinnerGrayNoVh)]
pub fn spinner_gray_no_vh() -> Html {
    return spinner(String::from("bg-gray"), String::new());
}

#[function_component(LoadingSpinnerDark)]
pub fn spinner_dark() -> Html {
    return spinner(String::from("bg-dark"), String::from("vh-100"));
}

fn spinner(background: String, vh: String) -> Html {
    return html!(
        <selection>
            <div class={format!("container-fluid p-5 {}", background)}>
                <div class="container">
                  <div class={format!("spinner-container {}", vh)}>
                    <div class="spinner"></div>
                  </div>
                </div>
            </div>
        </selection>
    );
}
