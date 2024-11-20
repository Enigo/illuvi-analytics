use log::warn;

pub fn scroll_to_top() {
    let window = web_sys::window();
    if window.is_some() {
        let options = web_sys::ScrollToOptions::new();
        options.set_top(0.0);
        options.set_behavior(web_sys::ScrollBehavior::Instant);
        window.unwrap().scroll_to_with_scroll_to_options(&options);
    } else {
        warn!("Window is not found!")
    }
}
