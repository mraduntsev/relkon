use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Primary,
    Outline,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Sm,
    Lg,
}

#[component]
pub fn CtaLink(
    href: &'static str,
    #[prop(into)] label: String,
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = Size::Lg)] size: Size,
    #[prop(default = false)] lift: bool,
    #[prop(default = "".to_string(), into)] class: String,
) -> impl IntoView {
    let variant_cls = match variant {
        Variant::Primary => "bg-accent text-ink font-bold hover:bg-accent-400",
        Variant::Outline => {
            "border border-steel-500/60 dark:border-steel-500/50 hover:border-accent hover:text-accent"
        }
    };
    let size_cls = match size {
        Size::Sm => "text-xs tracking-[0.16em] px-5 py-3",
        Size::Lg => "text-sm tracking-[0.14em] px-8 py-4",
    };
    let lift_cls = if lift { "hover:-translate-y-0.5" } else { "" };

    view! {
        <a href=href class=format!("btn-cut font-mono uppercase transition {variant_cls} {size_cls} {lift_cls} {class}")>
            {label}
        </a>
    }
}
