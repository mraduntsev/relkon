use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};

pub fn use_is_mobile() -> Signal<bool> {
    let (is_mobile, set_is_mobile) = signal(false);

    let update = move || {
        if let Some(window) = web_sys::window() {
            let width = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);
            set_is_mobile.set(width < 1024.0);
        }
    };
    update();

    if let Some(window) = web_sys::window() {
        let closure = Closure::wrap(Box::new(move || update()) as Box<dyn FnMut()>);
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget(); // утечка
    }

    is_mobile.into()
}
