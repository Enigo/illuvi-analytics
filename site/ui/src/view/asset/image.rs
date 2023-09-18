use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub image_url: String,
    pub burned: bool,
}

#[function_component(AssetImage)]
pub fn asset_image_function_component(props: &Props) -> Html {
    let name = &props.name;
    let image_url = &props.image_url;
    let burned = props.burned;
    return html! {
          <div class="bg-dark p-3 rounded border border-2 border-dark shadow-gradient text-center">
              <img src={image_url.clone()}
                class={format!("img-fluid rounded w-75 {}", if {burned} {"grayscale"} else {""})}
                loading="lazy" alt={name.clone()}/>
          </div>
    };
}
