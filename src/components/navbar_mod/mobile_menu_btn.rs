use leptos::prelude::*;

#[component]
pub fn MobileMenuBtn(
    on_click: Callback<()>,
    #[prop(optional)] is_open: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            aria-label="Меню"
            aria-expanded=move || is_open.get().to_string()
            class="lg:hidden w-10 h-10 border border-steel-500/50 dark:border-steel-600/60 flex flex-col items-center justify-center gap-1.5"
            on:click=move |_| on_click.run(())
        >
            <span class=move || {
                if is_open.get() {
                    "block h-[2px] w-5 bg-current transition rotate-45 translate-y-[3px]"
                } else {
                    "block h-[2px] w-5 bg-current transition"
                }
            }></span>
            <span class=move || {
                if is_open.get() {
                    "block h-[2px] w-5 bg-current transition -rotate-45 -translate-y-[3px]"
                } else {
                    "block h-[2px] w-5 bg-current transition"
                }
            }></span>
        </button>
    }
}
