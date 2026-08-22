use leptos::prelude::*;

use crate::{
    content::{SERVICES, Service},
    ui::SectionHeader,
};

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <section id="services" class="scroll-mt-24 py-24 md:py-32">
            <div class="max-w-7xl mx-auto px-5 md:px-8">
                <SectionHeader
                  number_service="Раздел 01 / Услуги".to_uppercase()
                  title="Что я проектирую"
                  description="Состав комплектов — по ГОСТ 21.501 и требованиям вашего техзадания." />

                <div class="border-t border-steel-500/40 dark:border-steel-600/40">
                    <div>
                        {SERVICES.iter().map(|service| {
                            view! { <ServiceCard service=service /> }
                        }).collect_view()}
                    </div>
                </div>
            </div>
      </section>
    }
}

#[component]
fn ServiceCard(service: &'static Service) -> impl IntoView {
    view! {
        <article class="group grid md:grid-cols-[90px_1.1fr_1fr_auto] gap-6 md:gap-10 py-10 border-b border-steel-500/40 dark:border-steel-600/40 hover:bg-white dark:hover:bg-ink-800 transition-colors px-2 md:px-4">
            <p class="font-display font-black text-3xl md:text-4xl text-steel-400/70 dark:text-steel-600 group-hover:text-accent transition-colors">
                { service.number }
            </p>
            <div>
                <h3 class="font-display font-bold text-xl md:text-2xl uppercase">
                    { service.title }
                </h3>
                <p class="mt-3 text-steel-600 dark:text-steel-400 leading-relaxed">
                    { service.description }
                </p>
                <div class="mt-4 flex flex-wrap gap-2">
                    {service.chips.iter().map(|chip| {
                        view! { <span class="chip">{ *chip }</span> }
                    }).collect_view()}
                </div>
            </div>
            <ul class="font-mono text-sm space-y-2 text-steel-600 dark:text-steel-400">
                {service.features.iter().map(|feature| {
                    view! { <li>{ *feature }</li> }
                }).collect_view()}
            </ul>
            <p class="font-mono text-xs text-steel-500 md:[writing-mode:vertical-rl] md:text-right">
                { service.sheet_number }
            </p>
        </article>
    }
}
