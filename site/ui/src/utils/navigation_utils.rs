use log::warn;

pub fn scroll_to_top() {
    let window = web_sys::window();
    if window.is_some() {
        window.unwrap().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new()
                .top(0.0)
                .behavior(web_sys::ScrollBehavior::Instant),
        );
    } else {
        warn!("Window is not found!")
    }
}
