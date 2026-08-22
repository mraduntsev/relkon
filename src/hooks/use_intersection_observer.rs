use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

pub fn use_intersection_observer(
    selector: &'static str,
    threshold: f64,
    once: bool,
    on_intersect: impl FnMut(Element) + 'static,
) {
    let on_intersect = Rc::new(RefCell::new(on_intersect));

    Effect::new(move |_| {
        let on_intersect = on_intersect.clone();
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let thresholds = js_sys::Array::new();
        thresholds.push(&JsValue::from_f64(threshold));
        let init = IntersectionObserverInit::new();
        init.set_threshold(&thresholds);

        let observer_slot: Rc<RefCell<Option<IntersectionObserver>>> = Rc::new(RefCell::new(None));
        // observer_slot.clone();

        let callback = Closure::wrap(Box::new(
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                for entry in entries {
                    if !entry.is_intersecting() {
                        continue;
                    }
                    if let Ok(element) = entry.target().dyn_into::<Element>() {
                        if once {
                            observer.unobserve(&element);
                        }
                        on_intersect.borrow_mut()(element);
                    }
                }
            },
        )
            as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);

        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &init)
                .expect("failed to create IntersectionObserver");

        if let Ok(elements) = document.query_selector_all(selector) {
            for i in 0..elements.length() {
                if let Some(node) = elements.get(i) {
                    if let Ok(element) = node.dyn_into::<Element>() {
                        let _ = observer.observe(&element);
                    }
                }
            }
        }

        *observer_slot.borrow_mut() = Some(observer.clone());

        let observer = SendWrapper::new(observer);
        let callback = SendWrapper::new(callback);

        on_cleanup(move || {
            observer.disconnect();
            drop(callback);
        });
    });
}
