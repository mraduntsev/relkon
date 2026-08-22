use leptos::prelude::*;

use crate::{
    content::{CHIP, STANDARDS, TOOLS, Tools},
    ui::SectionHeader,
};

#[component]
pub fn Tools() -> impl IntoView {
    view! {
        <section class="py-24 md:py-32 bg-paper-800 dark:bg-ink-900 border-y border-steel-500/30 dark:border-steel-600/40">
            <div class="max-w-7xl mx-auto px-5 md:px-8 grid lg:grid-cols-2 gap-14 lg:gap-20">

                <div>
                    <SectionHeader
                      number_service="Раздел 04.1 / Софт".to_uppercase()
                      title="Инструменты" />
                    <div class="mt-10 space-y-6">

                        {TOOLS.iter().map(|tools| {
                            view! { <LeftCard tools=tools /> }
                        }).collect_view()}
                    </div>
                    <div class="mt-10 flex flex-wrap gap-2">
                        {CHIP
                            .iter()
                            .map(|ch| view! { <span class="chip">{*ch}</span> })
                            .collect_view()}
                    </div>
                </div>
                <div>
                    <SectionHeader
                      number_service="Раздел 04.2 / Нормативы".to_uppercase()
                      title="База расчёта" />

                        <ul class="mt-10 divide-y divide-steel-500/30 dark:divide-steel-600/40 border-y border-steel-500/30 dark:border-steel-600/40 font-mono text-sm">
                            {STANDARDS
                                .iter()
                                .map(|std| right_card_item(*std))
                                .collect_view()}
                        </ul>
        <p class="mt-6 font-mono text-xs text-steel-500 leading-relaxed">* Редакции отслеживаю по реестру действующих нормативов. Расчёт только по актуальным сводам правил.</p>
                </div>
            </div>
      </section>
    }
}

#[component]
fn LeftCard(tools: &'static Tools) -> impl IntoView {
    view! {
        <div class="flex justify-between font-mono text-xs uppercase tracking-[0.16em] mb-2">
            <span>{tools.title}</span>
            <span class="text-accent">{tools.level}%</span>
        </div>

        <div class="h-1.5 bg-steel-500/30 dark:bg-steel-600/40">
            <div class="bar-fill h-full bg-accent" data-level={tools.level}></div>
        </div>
    }
}

fn right_card_item(title: &'static str) -> impl IntoView {
    view! {
        <li class="py-4 flex items-center justify-between gap-4">
            <span>{title}</span>
            <span class="text-accent">"✓"</span>
        </li>
    }
}
