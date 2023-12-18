#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "fixed top-1/4 right-0 px-4 py-3",
            div { class: "grid grid-rows-4 w-fit gap-x-4",
                a { href: "#", "About Me" }
                a { href: "#", "Profolio" }
                a { href: "#", "Work" }
            }
        }
    })
}
