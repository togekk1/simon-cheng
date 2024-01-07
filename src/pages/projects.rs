use dioxus::prelude::{fc_to_builder, rsx, Element, Scope};

#[allow(non_snake_case)]
pub fn Projects(cx: Scope) -> Element {
    cx.render(rsx! { crate::pages::layout::Layout { title: "Projects" } })
}
