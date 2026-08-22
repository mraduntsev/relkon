use leptos::prelude::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <a href="#top" class="flex items-center gap-3 group">
            <span class="w-9 h-9 bg-accent text-ink font-display font-bold text-sm flex items-center justify-center btn-cut group-hover:translate-y-[-2px] transition">
                "Р"
            </span>
            <span class="leading-none">
                <span class="block font-display font-bold text-sm md:text-base tracking-wide">
                    "РАДУНЦЕВ·КМ"
                </span>
                <span class="block font-mono text-[10px] uppercase tracking-[0.22em] text-steel-500 dark:text-steel-400 mt-1">
                    "конструктор КМ / КЖ / КМД"
                </span>
            </span>
        </a>
    }
}
