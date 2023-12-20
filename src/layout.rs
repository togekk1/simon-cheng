use crate::enums::route::Route;
use dioxus::prelude::{component, fc_to_builder, rsx, Element, Scope};
use dioxus_router::prelude::Outlet;

#[component]
#[allow(non_snake_case)]
pub fn Layout(cx: Scope) -> Element {
    cx.render(rsx! {
        crate::components::navbar::Navbar {}
        Outlet::<Route> {}
    })
}
