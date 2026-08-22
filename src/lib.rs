pub mod components;
pub mod content;
pub mod hooks;
pub mod ui;

use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::{
    About, Contacts, Footer, Hero, Navbar, Process, Projects, Services, Ticker, Tools,
};

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen text-gray-900 dark:bg-gray-900 dark:text-white">
            <Navbar />
            <main id="top">
                <Hero />
                <Ticker />
                <Services />
                <Projects />
                <Process />
                <Tools />
                <About />
                <Contacts />
                <Footer />
            </main>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}
