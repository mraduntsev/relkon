use leptos::prelude::*;

#[component]
pub fn Icon(
    #[prop(into)] name: String,
    #[prop(default = "18")] size: &'static str,
    #[prop(default = "currentColor")] color: &'static str,
    #[prop(default = "2")] stroke_width: &'static str,
    #[prop(default = "".to_string(), into)] class: String,
) -> impl IntoView {
    match name.as_str() {
        "sun" => view! {
            <svg class=format!("sun-icon hidden dark:block {class}") xmlns="http://www.w3.org/2000/svg"
                width=size height=size viewBox="0 0 24 24" fill="none" stroke=color
                stroke-width=stroke_width stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="4"/>
                <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
            </svg>
        }.into_any(),
        "moon" => view! {
            <svg class=format!("moon-icon block dark:hidden {class}") xmlns="http://www.w3.org/2000/svg"
                width=size height=size viewBox="0 0 24 24" fill="none" stroke=color
                stroke-width=stroke_width stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        }.into_any(),
        _ => view! { <svg width=size height=size/> }.into_any(),
    }
}
