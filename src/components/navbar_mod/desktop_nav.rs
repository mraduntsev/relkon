use leptos::prelude::*;

use crate::{
    content::{NAV_ITEMS, nav_ids},
    hooks::use_scroll_spy,
};

#[component]
pub fn DesktopNav() -> impl IntoView {
    let active = use_scroll_spy(nav_ids(), 0.1);

    view! {
        <nav class="hidden lg:flex items-center gap-8 font-mono text-xs uppercase tracking-[0.18em]">
            {NAV_ITEMS.iter().map(|item| {
                let id = &item.href[1..];
                let id_string = id.to_string();
                view! {
                    <a
                        href=item.href
                        class="navlink hover:text-accent transition-colors"
                        class:text-accent=move || active.get().as_deref() == Some(id_string.as_str())
                        data-spy
                    >
                        {item.label}
                    </a>
                }
            }).collect_view()}
        </nav>
    }
}
