use leptos::prelude::*;

use crate::hooks::use_intersection_observer;

pub fn use_scroll_spy(
    section_selectors: Vec<&'static str>,
    threshold: f64,
) -> ReadSignal<Option<String>> {
    use_intersection_observer(".reveal", threshold, true, |element| {
        let _ = element.class_list().add_1("visible");
    });

    let (active_section, set_active_section) = signal(None::<String>);
    for selector in section_selectors {
        use_intersection_observer(selector, threshold, false, move |element| {
            let id = element.id();
            if !id.is_empty() {
                set_active_section.set(Some(id));
            }
        });
    }

    active_section
}
