use leptos::prelude::*;

use crate::{
    components::FrameSchematic,
    content::{AXIS_LABELS, STAMP_FIELDS, STATS, StatItem},
    ui::{CtaLink, Variant},
};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative bp-grid pt-28 md:pt-40 pb-10 md:pb-14 overflow-hidden">
            <AxisLabels />
            <div class="max-w-7xl mx-auto px-5 md:px-8">
                <div class="grid lg:grid-cols-12 gap-12 lg:gap-8 items-center">
                    <LeftColumn />
                    <RightColumn />
                </div>
                <StampSheet />
            </div>
        </section>
    }
}

#[component]
pub fn LeftColumn() -> impl IntoView {
    view! {
        <div class="lg:col-span-6">
            <StatusBadge />
            <h1 class="mt-6 font-display font-black text-[clamp(2rem,5.4vw,4.2rem)] leading-[1.04] uppercase">
                "Металл держится" <br/>
                <span class="text-accent">"на расчёте"</span>
            </h1>
            <p class="rv mt-6 max-w-xl text-base md:text-lg text-steel-600 dark:text-steel-400 leading-relaxed">
                "Проектирую каркасы зданий и сооружений: "
                <strong class="text-[#10161F] dark:text-steel-200">"КМ, КЖ, КМД"</strong>
                ". От расчётной схемы до деталировочных чертежей, которые завод собирает без единого вопроса."
            </p>
            <CtaButtons />
            <StatsGrid />
        </div>
    }
}

#[component]
fn RightColumn() -> impl IntoView {
    view! {
        <div class="lg:col-span-6">
            <FrameSchematic />
        </div>
    }
}

#[component]
pub fn AxisLabels() -> impl IntoView {
    view! {
        <div class="hidden lg:flex absolute top-24 left-0 flex-col gap-40 font-mono text-[11px] text-steel-500 dark:text-steel-400 select-none" aria-hidden="true">
            {AXIS_LABELS.iter().map(|label| {
                view! {
                    <span class="w-7 h-7 border border-current rounded-full flex items-center justify-center">
                        {label.to_string()}
                    </span>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn StampSheet() -> impl IntoView {
    view! {
        <div class="mt-14 border-2 border-[#10161F]/60 dark:border-steel-400/50 grid grid-cols-2 md:grid-cols-4 font-mono text-[10px] md:text-[11px] uppercase tracking-[0.14em]">
            {STAMP_FIELDS.iter().map(|field| {
                view! {
                    <div class="p-3 border-b border-r border-steel-500/40 dark:border-steel-500/30">
                        <span class="text-steel-500">{field.label}</span> {field.value}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn StatusBadge() -> impl IntoView {
    view! {
        <p class="font-mono text-[11px] md:text-xs uppercase tracking-[0.3em] text-steel-500 dark:text-steel-400 flex items-center gap-3">
            <span class="pulse-dot inline-block w-2 h-2 rounded-full bg-green-500"></span>
            "ИП Радунцев М. В. · открыт к проектам"
        </p>
    }
}

#[component]
fn CtaButtons() -> impl IntoView {
    view! {
        <div class="mt-8 flex flex-wrap gap-4">
            <CtaLink href="#contact" label="Обсудить проект" lift=true/>
            <CtaLink href="#projects" label="Смотреть работы ↓" variant=Variant::Outline lift=true/>
        </div>
    }
}

#[component]
fn StatsGrid() -> impl IntoView {
    view! {
        <div class="mt-12 grid grid-cols-2 sm:grid-cols-4 gap-px bg-steel-500/40 dark:bg-steel-600/40 border border-steel-500/40 dark:border-steel-600/40">
            {STATS.iter().map(|stat| {
                view! { <StatItem stat=stat /> }
            }).collect_view()}
        </div>
    }
}

#[component]
fn StatItem(stat: &'static StatItem) -> impl IntoView {
    view! {
        <div class="bg-paper dark:bg-ink-800 p-4">
            <p class="font-mono font-bold text-2xl md:text-3xl text-[#10161F] dark:text-white">
                <span data-count=stat.value>{stat.value}</span>{stat.suffix}
            </p>
            <p class="mt-1 font-mono text-[10px] uppercase tracking-[0.18em] text-steel-500">
                {stat.label}
            </p>
        </div>
    }
}
