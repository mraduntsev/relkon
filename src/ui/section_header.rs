use leptos::prelude::*;

#[component]
pub fn SectionHeader(
    number_service: String,
    title: &'static str,
    #[prop(default = "".to_string(), into)] description: String,
    #[prop(default = false)] centered: bool,
) -> impl IntoView {
    if centered {
        view! {
            <p class="sec-label">
                <span class="h-px w-10 bg-accent"></span>
                { number_service }
            </p>
            <h2 class="mt-4 font-display font-bold text-3xl md:text-5xl uppercase leading-tight">
                { title }
            </h2>
        }
        .into_any()
    } else {
        view! {
            <div class="flex items-end justify-between gap-6 mb-12 md:mb-16">
                <div>
                    <p class="sec-label">
                        <span class="h-px w-10 bg-accent"></span>
                        { number_service }
                    </p>
                    <h2 class="mt-4 font-display font-bold text-3xl md:text-5xl uppercase leading-tight">
                        { title }
                    </h2>
                </div>
                <p class="hidden md:block font-mono text-xs text-steel-500 max-w-[240px] text-right leading-relaxed">
                    { description }
                </p>
            </div>
        }.into_any()
    }
}
