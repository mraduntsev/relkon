use leptos::prelude::*;

use crate::{content::PROFILE, ui::SectionHeader};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="scroll-mt-24 py-24 md:py-32">
            <div class="max-w-7xl mx-auto px-5 md:px-8 grid lg:grid-cols-12 gap-12 lg:gap-16 items-start">

                <LeftCard />
                <RightCard />

            </div>
      </section>
    }
}

#[component]
fn RightCard() -> impl IntoView {
    view! {
        <div class="lg:col-span-7">
            <SectionHeader
                  number_service="Раздел 05 / Обо мне".to_uppercase()
                  title="Конструктор, который отвечает за каждый узел" />
            <div class="mt-8 space-y-5 text-base md:text-lg text-steel-600 dark:text-steel-400 leading-relaxed max-w-2xl">
                {PROFILE.about.iter().map(|paragraph| {
                    view! {
                        <p>
                            {paragraph.to_string()}
                        </p>
                    }
                }).collect_view()}
            </div>
                <blockquote class="mt-10 border-l-4 border-accent pl-6 py-2 max-w-2xl">
                  <p class="font-display text-lg md:text-2xl leading-snug">{PROFILE.quote}</p>
                  <footer class="mt-4 flex items-center gap-4">
                    <svg viewBox="0 0 160 48" class="h-9 text-accent" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M8 34 C 22 10, 30 12, 34 26 C 37 36, 44 34, 50 22 C 55 12, 62 14, 64 24 C 66 33, 76 32, 84 20 M 96 30 C 104 12, 112 14, 112 24 C 112 34, 122 32, 130 20 C 138 10, 146 16, 152 26"/></svg>
                    <span class="font-mono text-xs uppercase tracking-[0.2em] text-steel-500">{PROFILE.signature}</span>
                  </footer>
                </blockquote>
        </div>
    }
}

#[component]
fn LeftCard() -> impl IntoView {
    view! {
        <div class="lg:col-span-5">
            <div class="kb relative border-2 border-[#10161F]/50 dark:border-steel-400/40 p-2">
              <span class="absolute -top-px -left-px w-6 h-6 border-t-4 border-l-4 border-accent"></span>
              <span class="absolute -bottom-px -right-px w-6 h-6 border-b-4 border-r-4 border-accent"></span>
                  <div class="overflow-hidden aspect-[4/5]">
                    <img
                        src={PROFILE.src}
                        alt={PROFILE.full_name}
                        class="w-full h-full object-cover" />
                  </div>
                  <div class="flex items-center justify-between px-3 py-3 font-mono text-[10px] md:text-[11px] uppercase tracking-[0.18em] text-steel-500">
                    <span>{PROFILE.full_name}</span><span class="text-accent">{PROFILE.position}</span>
                  </div>
            </div>
            <div class="grid grid-cols-1 gap-px font-mono text-[11px] uppercase tracking-[0.14em]">

                <QualificationsAndWorkInfo />

            </div>
        </div>
    }
}

#[component]
pub fn QualificationsAndWorkInfo() -> impl IntoView {
    view! {
        <div class="mt-6 space-y-6">
            <section>
                <h2 class="section-title">"Условия работы"</h2>
                <div class="mt-3 grid grid-cols-2 gap-px bg-steel-500/40 dark:bg-steel-600/40 border border-steel-500/40 dark:border-steel-600/40 font-mono text-[11px] uppercase tracking-[0.14em]">
                    {PROFILE.work_info.iter().map(|info| {
                        view! {
                            <div class="bg-paper dark:bg-ink-800 p-3">
                                {info.to_string()}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section>
                <h2 class="section-title">"Квалификация"</h2>
                <div class="mt-3 grid grid-cols-1 gap-px bg-steel-500/40 dark:bg-steel-600/40 border border-steel-500/40 dark:border-steel-600/40 font-mono text-[11px] uppercase tracking-[0.14em]">
                    {PROFILE.qualifications.iter().map(|q| {
                        let mut text = q.title.to_string();
                        if let Some(org) = q.organization {
                            text.push_str(&format!(" ({})", org));
                        }
                        if let Some(year) = q.year {
                            text.push_str(&format!(", {}", year));
                        }

                        view! {
                            <div class="bg-paper dark:bg-ink-800 p-3 flex items-center gap-2">
                                <span class="text-accent">"✦"</span>
                                <span>{text}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>
        </div>
    }
}
