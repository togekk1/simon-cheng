use crate::pages::about_me::AboutMe;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::portfolio::Portfolio;
use crate::pages::projects::Projects;
use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, rsx, Element, GlobalAttributes, Scope,
};
use dioxus_router::prelude::{Link, Outlet, Routable, ToRouteSegments};

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    AboutMe {},
    #[route("/portfolio")]
    Portfolio {},
    #[route("/projects")]
    Projects {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
#[allow(non_snake_case)]
fn Layout(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {}
        Outlet::<Route> {}
    })
}

#[allow(non_snake_case)]
fn Navbar(cx: Scope) -> Element {
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
