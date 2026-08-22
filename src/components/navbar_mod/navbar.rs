use leptos::prelude::*;

use crate::{
    components::{
        MobileMenuBtn,
        navbar_mod::{DesktopNav, Logo, MobileDrawer, ThemeToggle},
    },
    hooks::*,
    ui::{Brand, BrandVariant, CtaLink, Size},
};

#[component]
pub fn Navbar() -> impl IntoView {
    let _theme = use_theme();
    let _is_mobile = use_is_mobile();
    let (drawer_open, set_drawer_open) = signal(false);

    let toggle_drawer = Callback::new(move |_| set_drawer_open.update(|v| *v = !*v));
    let close_drawer = Callback::new(move |_| set_drawer_open.set(false));

    view! {
        <header class="fixed inset-x-0 top-0 z-50 border-b border-steel-500/30 dark:border-steel-600/40 bg-paper/85 dark:bg-ink/85 backdrop-blur-md">
            <div class="max-w-7xl mx-auto px-5 md:px-8 h-16 md:h-[72px] flex items-center justify-between gap-4">
                <Brand variant={BrandVariant::Compact { href: Some("#top".to_string()) }} />

                <DesktopNav />

                <div class="flex items-center gap-3">
                    <ThemeToggle />
                    <CtaLink href="#contact" label="Обсудить проект" size=Size::Sm class="hidden md:inline-flex".to_string()/>

                    <MobileMenuBtn
                        on_click=toggle_drawer
                        is_open=Signal::from(drawer_open)
                    />
                </div>
            </div>

            <MobileDrawer
                is_open=Signal::from(drawer_open)
                on_close=close_drawer
            />
        </header>
    }
}
