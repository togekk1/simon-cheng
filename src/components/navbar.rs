use crate::enums::route::Route;
use dioxus::prelude::{dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, Scope};
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "fixed top-1/4 right-0 px-6 py-3",
            div { class: "grid grid-rows-4 w-fit gap-x-4 text-gray-900",
                Link { to: Route::AboutMe {}, "About Me" }
                Link { to: Route::Portfolio {}, "Portfolio" }
                Link { to: Route::Projects {}, "Projects" }
            }
        }
    })
}
