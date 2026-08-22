use leptos::prelude::*;

use crate::content::TICKER;

#[component]
pub fn Ticker() -> impl IntoView {
    let render_items = || {
        TICKER
            .iter()
            .map(|tick| {
                view! {
                    <span class="px-6">{tick.to_string()}</span>
                    <span class="text-accent">"✦"</span>
                }
            })
            .collect_view()
    };
    view! {
        <div class="ticker border-y border-steel-500/30 dark:border-steel-600/40 bg-paper-800 dark:bg-ink-800 overflow-hidden py-3 select-none" aria-hidden="true">
            <div class="ticker-track flex w-max gap-0 font-mono text-xs uppercase tracking-[0.24em] text-steel-600 dark:text-steel-400">
                <div class="flex shrink-0" aria-hidden="false">
                    {render_items()}
                </div>
                <div class="flex shrink-0" aria-hidden="true">
                    {render_items()}
                </div>
            </div>
        </div>
    }
}
