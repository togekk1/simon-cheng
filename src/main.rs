mod components;
mod enums;
mod layout;
mod pages;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::{component, fc_to_builder, rsx, Element, Scope};
use dioxus_router::prelude::Router;
// use serde::{Deserialize, Serialize};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

#[component]
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! { Router::<enums::route::Route> {} })
}
