use leptos::prelude::*;

use crate::{
    content::{PROJECTS, Project},
    ui::SectionHeader,
};

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="scroll-mt-24 py-24 md:py-32 bg-paper-800 dark:bg-ink-900 border-y border-steel-500/30 dark:border-steel-600/40">
            <div class="max-w-7xl mx-auto px-5 md:px-8">
                <SectionHeader
                  number_service="Раздел 02 / Портфолио".to_uppercase()
                  title="Избранные проекты"
                  description="Каждый комплект прошёл экспертизу или собран на площадке без замечаний." />

                <div class="grid md:grid-cols-2 xl:grid-cols-3 gap-6">
                    {PROJECTS.iter().map(|project| {
                        view! { <ProjectCard project=project /> }
                    }).collect_view()}
                </div>
            </div>
      </section>
    }
}

#[component]
fn ProjectCard(project: &'static Project) -> impl IntoView {
    view! {
        <article class="group relative border border-steel-500/40 dark:border-steel-600/50 bg-white dark:bg-ink-800 hover:border-accent transition-colors">
            <ProjectImage project=project />
            <div class="p-5 md:p-6">
                <p class="font-mono text-[11px] uppercase tracking-[0.18em] text-steel-500 flex justify-between">
                    <span>{project.number}</span>
                    <span>{project.year}</span>
                </p>
                <h3 class="mt-2 font-display font-bold text-lg uppercase leading-snug group-hover:text-accent transition-colors">
                    {project.title}
                </h3>
                <p class="mt-2 text-sm text-steel-600 dark:text-steel-400">
                    {project.description}
                </p>
                <ProjectFeatures features=project.features />
            </div>
        </article>
    }
}

#[component]
fn ProjectImage(project: &'static Project) -> impl IntoView {
    view! {
        <div class="relative overflow-hidden aspect-[4/3]">
            <img
                src=project.src
                alt=project.title
                loading="lazy"
                class="w-full h-full object-cover transition-transform duration-[1200ms] ease-out group-hover:scale-[1.07]"
            />
            <span class="absolute top-3 left-3 font-mono text-[10px] uppercase tracking-[0.16em] bg-ink/80 text-paper px-2 py-1">
                {project.sheet_number}
            </span>
            <div class="absolute bottom-3 left-3 flex gap-2">
                {project.chips.into_iter().map(|chip| {
                    view! { <span class="chip !bg-ink/80 !text-paper !border-transparent">{*chip}</span> }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn ProjectFeatures(features: &'static [&'static str]) -> impl IntoView {
    let chips: Vec<_> = features
        .iter()
        .map(|feature| {
            view! {
                <span class="chip !bg-ink/80 !text-paper !border-transparent">{*feature}</span>
            }
        })
        .collect();

    view! {
        <p class="mt-4 pt-4 border-t border-steel-500/30 dark:border-steel-600/40 font-mono text-xs text-steel-500">
            {match chips.len() {
                1 => chips[0].clone().into_any(),

                2 => view! {
                    {chips[0].clone()}
                    <span class="mx-1 text-ink/50">" · "</span>
                    {chips[1].clone()}
                }.into_any(),

                _ => ().into_any(),
            }}
        </p>
    }
}
