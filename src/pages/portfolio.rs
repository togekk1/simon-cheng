use dioxus::prelude::{dioxus_elements, rsx, Element, Scope,GlobalAttributes};

#[allow(non_snake_case)]
pub fn Portfolio(cx: Scope) -> Element {
    cx.render(rsx! { 
                div { class: "bg-black h-screen bg-[url('img/IMG_20230423_150135_HDR.jpg')] bg-cover",
                div { "Portfolio" }
        }

})
}