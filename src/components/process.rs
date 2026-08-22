use leptos::prelude::*;

use crate::{
    content::{PROCESS, Process},
    ui::SectionHeader,
};

#[component]
pub fn Process() -> impl IntoView {
    view! {
        <section id="process" class="scroll-mt-24 py-24 md:py-32 bg-paper-800 dark:bg-ink-900 border-y border-steel-500/30 dark:border-steel-600/40">
            <div class="max-w-7xl mx-auto px-5 md:px-8">
                <SectionHeader
                  number_service="Раздел 03 / Регламент".to_uppercase()
                  title="Как идёт работа"
                  description="Прозрачные этапы и контрольные точки на каждом листе." />

                <ol class="relative border-l-2 border-dashed border-steel-500/50 dark:border-steel-600/60 ml-4 md:ml-8 space-y-12">
                    {PROCESS.iter().map(|process| {
                        view! { <ProcessCard process=process /> }
                    }).collect_view()}
                </ol>
            </div>
      </section>
    }
}

#[component]
fn ProcessCard(process: &'static Process) -> impl IntoView {
    view! {
        <li class="relative pl-10 md:pl-14">
        <span class="absolute -left-[21px] top-0 w-10 h-10 rounded-full border-2 border-accent bg-paper dark:bg-ink font-mono text-xs font-bold flex items-center justify-center">{process.number}</span>

        <div class="flex flex-wrap items-baseline gap-x-6 gap-y-2">
            <h3 class="font-display font-bold text-lg md:text-xl uppercase">{process.title}</h3>
            <span class="font-mono text-xs text-accent uppercase tracking-[0.16em]">{process.time}</span>
          </div>

        <p class="mt-2 max-w-2xl text-steel-600 dark:text-steel-400">{process.description}</p>
        </li>
    }
}
