use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

const STORAGE_KEY: &str = "theme";

fn apply_theme_class(is_dark: bool) {
    let Some(html) = window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };

    if is_dark {
        let _ = html.class_list().add_1("dark");
    } else {
        let _ = html.class_list().remove_1("dark");
    }
}

fn initial_theme() -> bool {
    if let Ok(saved) = LocalStorage::get::<String>(STORAGE_KEY) {
        return saved == "dark";
    }

    window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .map(|media_query| media_query.matches())
        .unwrap_or(false)
}

pub fn use_theme() -> (ReadSignal<bool>, WriteSignal<bool>) {
    let (is_dark, set_is_dark) = signal(initial_theme());

    Effect::new(move |_| {
        let dark = is_dark.get();

        apply_theme_class(dark);

        let theme = if dark { "dark" } else { "light" };

        let _ = LocalStorage::set(STORAGE_KEY, theme);
    });

    (is_dark, set_is_dark)
}
