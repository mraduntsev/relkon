use leptos::prelude::*;

use crate::{hooks::use_theme, ui::Icon};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = use_theme();

    let toggle = move |_| {
        set_is_dark.update(|dark| *dark = !*dark);
    };

    view! {
        <button on:click=toggle>
            <Icon name="sun"/>
            <Icon name="moon" size="24" class="ml-2"/>
        </button>
    }
}
