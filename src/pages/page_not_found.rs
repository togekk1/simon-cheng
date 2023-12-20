use dioxus::prelude::{component, dioxus_elements, rsx, Element, GlobalAttributes, Props, Scope};

#[component]
#[allow(non_snake_case)]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    cx.render(rsx! {
        div {"404"}
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    })
}
