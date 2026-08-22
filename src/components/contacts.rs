use leptos::prelude::*;

use crate::{content::CONTACTS, ui::SectionHeader};

#[component]
pub fn Contacts() -> impl IntoView {
    view! {
        <section id="contact" class="scroll-mt-24 py-24 md:py-32 bp-grid border-t border-steel-500/30 dark:border-steel-600/40">
            <div class="max-w-7xl mx-auto px-5 md:px-8">

            <SectionHeader
                      number_service="Раздел 06 / Связь".to_uppercase()
                      title="Обсудим ваш каркас?"
            description="По телефону можно просто на словах описать задачу — этого достаточно для начала. Документы и эскизы удобнее сразу отправить на почту."
            />

                <div class="grid lg:grid-cols-12 gap-12 lg:gap-16">

                <LeftCard />
                <RightCard />

                </div>

            </div>
        </section>
    }
}

#[component]
fn LeftCard() -> impl IntoView {
    view! {
        <div class="lg:col-span-5">
            {CONTACTS.contacts.iter().map(|contacts| {
                view! {
                    <div class="group flex items-center justify-between gap-4 border border-steel-500/40 dark:border-steel-600/50 px-6 py-5 hover:border-accent transition">

                    <span>
                      <span class="block font-mono text-[10px] uppercase tracking-[0.24em] text-steel-500">{contacts.title}</span>
                      <span class="block mt-1.5 font-mono text-sm md:text-base">{contacts.object}</span>
                    </span>
                    </div>
                        }
                    }).collect_view()}
        </div>
    }
}

#[component]
fn RightCard() -> impl IntoView {
    view! {
        <div class="lg:col-span-7">
            <p class="sec-label"><span class="h-px w-10 bg-accent"></span>Регламент старта</p>

                <div class="mt-6 grid sm:grid-cols-2 lg:grid-cols-4 gap-px bg-steel-500/40 dark:bg-steel-600/40 border border-steel-500/40 dark:border-steel-600/40">
                {CONTACTS.points.iter().map(|points| {
                    view! {
                        <div class="bg-paper dark:bg-ink-800 p-5">

                            <p class="font-display font-black text-2xl text-accent">{points.number}</p>
                            <p class="mt-2 font-mono text-xs font-bold uppercase tracking-[0.16em]">{points.title}</p>
                            <p class="mt-2 text-sm text-steel-600 dark:text-steel-400 leading-relaxed">{points.description}</p>

                        </div>
                            }
                        }).collect_view()}
            </div>
        </div>
    }
}
