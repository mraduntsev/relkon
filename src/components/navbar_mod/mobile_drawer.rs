use leptos::prelude::*;

use crate::content::NAV_ITEMS;

#[component]
pub fn MobileDrawer(is_open: Signal<bool>, on_close: Callback<()>) -> impl IntoView {
    view! {
        <Show when=move || is_open.get()>
            <div class="lg:hidden border-t border-steel-500/30 dark:border-steel-600/40 bg-paper dark:bg-ink-800">
                <nav class="px-5 py-4 flex flex-col gap-1 font-mono text-sm uppercase tracking-[0.18em]">
                    {NAV_ITEMS.iter().map(|item| {
                        let href = item.href;
                        let label = item.label;
                        let close = on_close.clone();
                        view! {
                            <a
                                href=href
                                class="mnav py-3 border-b border-steel-500/20 dark:border-steel-600/30 hover:text-accent"
                                on:click=move |_| close.run(())
                            >
                                {label}
                            </a>
                        }
                    }).collect_view()}
                </nav>
            </div>
        </Show>
    }
}
