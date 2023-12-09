use web_sys::{Element, Event};
use yew::Callback;

pub fn get_image_onload_callback(div: Element) -> Callback<Event> {
    return Callback::from(move |_| div.remove());
}

pub fn create_image_overlay_element() -> Element {
    return create_image_overlay_with_width_element(String::from("w-100"));
}

fn create_image_overlay_with_width_element(width: String) -> Element {
    let div = create_div_element();
    div.set_attribute("class", format!("loading-overlay {}", width).as_str())
        .unwrap();
    return div;
}

fn create_div_element() -> Element {
    return web_sys::window()
        .expect("Can't find window")
        .document()
        .expect("Can't find document")
        .create_element("div")
        .unwrap();
}
