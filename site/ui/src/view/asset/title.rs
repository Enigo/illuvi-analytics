use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub burned: bool,
}

#[function_component(AssetTitle)]
pub fn asset_title_function_component(props: &Props) -> Html {
    let name = &props.name;
    let burned = props.burned;
    return html! {
        <div class="row align-items-center justify-content-lg-start justify-content-center text-center text-lg-start">
          <div class="col-md-auto">
            <p class="text-white fs-2 my-2">{name}</p>
          </div>
          if {burned} {
              <div class="col-md-auto">
                    <i style="color: #ff0000; font-size: 1.75rem;" class="fa-solid fa-fire"></i>
              </div>
          }
        </div>
    };
}
