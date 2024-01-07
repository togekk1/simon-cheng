use dioxus::prelude::{
    dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, Props, Scope,
};

#[derive(Props)]
pub struct TitleProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
pub fn Layout<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    cx.render(rsx! {
        div { class: "bg-black h-screen bg-[url('img/home-bg.avif')] bg-cover bg-center pt-16",
            crate::components::title::Title { title: cx.props.title }
        }
    })
}
