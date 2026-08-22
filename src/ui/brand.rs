use std::string::ToString;

use leptos::{either::Either, prelude::*};

use crate::content::BRAND;

#[derive(Clone, PartialEq)]
pub enum BrandVariant {
    Full,
    Compact { href: Option<String> },
}

#[component]
pub fn Brand(variant: BrandVariant) -> impl IntoView {
    match variant {
        BrandVariant::Full => Either::Left(view! {
            <div>
                <p class="font-display font-bold text-lg">{BRAND.name}</p>
                <p class="mt-2 font-mono text-xs text-steel-500 leading-relaxed">
                    {BRAND.full_desc_line1}<br/>{BRAND.full_desc_line2}
                </p>
            </div>
        }),
        BrandVariant::Compact { href } => {
            let href = href.unwrap_or_else(|| "#top".to_string());
            Either::Right(view! {
                <a href={href} class="flex items-center gap-3 group">
                    <span class="w-9 h-9 bg-accent text-ink font-display font-bold text-sm flex items-center justify-center btn-cut group-hover:translate-y-[-2px] transition">
                        {BRAND.initial}
                    </span>
                    <span class="leading-none">
                        <span class="block font-display font-bold text-sm md:text-base tracking-wide">
                            {BRAND.name}
                        </span>
                        <span class="block font-mono text-[10px] uppercase tracking-[0.22em] text-steel-500 dark:text-steel-400 mt-1">
                            {BRAND.compact_desc}
                        </span>
                    </span>
                </a>
            })
        }
    }
}
