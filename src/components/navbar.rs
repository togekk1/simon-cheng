use crate::enums::route::Route;
use dioxus::prelude::{dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, Scope};
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "fixed h-screen top-0 right-0 text-gray-900 bg-gradient-to-l from-[rgba(0,_0,_0,_0.7)] bg-[200px] hover:bg-center bg-no-repeat hover:text-gray-300 transition-all duration-700",
            div { class: "mt-[25vh] w-[200px] py-3",
            div {
                class: "grid grid-rows-4 gap-x-4 w-fit mx-auto pl-4",
                Link { to: Route::AboutMe {}, class:"hover:text-white", "About Me" }
                Link { to: Route::Portfolio {}, class:"hover:text-white", "Portfolio" }
                Link { to: Route::Projects {}, class:"hover:text-white", "Projects" }

            }
            }
        }
    })
}
