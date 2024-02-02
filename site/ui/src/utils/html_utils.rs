use web_sys::{Element, Event};
use yew::Callback;

pub fn get_image_onload_callback(div: Element) -> Callback<Event> {
    return Callback::from(move |_| div.remove());
}

pub fn create_image_overlay_element() -> Element {
    let div = create_div_element();
    div.set_attribute("class", "loading-overlay").unwrap();
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
