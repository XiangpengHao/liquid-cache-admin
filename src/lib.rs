use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod models;
mod pages;
mod utils;

use crate::components::toast::ToastProvider;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Title text="Liquid Cache Admin" />

        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <ToastProvider>
            <Router>
                <Routes fallback=|| view! { NotFound }>
                    <Route path=path!("/") view=Home />
                </Routes>
            </Router>
        </ToastProvider>
    }
}
