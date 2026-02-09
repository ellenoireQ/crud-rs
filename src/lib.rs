use leptos::prelude::*;

pub mod models;
pub mod components;
pub mod pages;

use pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Home />
    }
}