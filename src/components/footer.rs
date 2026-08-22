use leptos::prelude::*;

use crate::{
    content::{FOOTER, NAV_ITEMS},
    ui::{Brand, BrandVariant},
};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-steel-500/30 dark:border-steel-600/40 bg-paper-800 dark:bg-ink-900">
          <div class="max-w-7xl mx-auto px-5 md:px-8 py-12 grid md:grid-cols-3 gap-10 items-start">
            <div>
              <Brand variant={BrandVariant::Full} />
            </div>
            <nav class="flex flex-wrap gap-x-6 gap-y-2 font-mono text-xs uppercase tracking-[0.18em]">
                  {NAV_ITEMS.iter().map(|item| {
                    view! {
                        <a
                            href=item.href
                            class="hover:text-accent transition"
                        >
                            {item.label}
                        </a>
                    }
                }).collect_view()}
            </nav>
            <div class="font-mono text-[10px] uppercase tracking-[0.16em] border border-steel-500/40 dark:border-steel-600/40 grid grid-cols-2">
                    {FOOTER.work_info.iter().map(|info| {
                        view! {
                            <span class="p-2 border border-steel-500/40 dark:border-steel-600/40 text-steel-500">
                                {info.to_string()}
                            </span>
                        }
                    }).collect_view()}
            </div>
          </div>
          <div class="border-t border-steel-500/30 dark:border-steel-600/40">
            <div class="max-w-7xl mx-auto px-5 md:px-8 py-5 flex flex-wrap items-center justify-between gap-3 font-mono text-[11px] text-steel-500">
              <span>{FOOTER.copyright}</span>
            </div>
          </div>
        </footer>
        <a href="#top" aria-label="Наверх" class="fixed bottom-6 right-6 z-50 w-12 h-12 flex items-center justify-center border border-steel-500/60 dark:border-steel-500/50 bg-paper dark:bg-ink-800 font-mono text-lg hover:border-accent hover:text-accent transition">"↑"</a>
    }
}
