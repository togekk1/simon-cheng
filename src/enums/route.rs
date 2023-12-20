use crate::pages::about_me::AboutMe;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::portfolio::Portfolio;
use crate::pages::projects::Projects;
use dioxus::prelude::{fc_to_builder, render};
use dioxus_router::prelude::{Routable, ToRouteSegments};

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(crate::layout::Layout)]
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
