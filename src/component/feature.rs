use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css(style, "src/component/feature.scss")]
#[component]
pub fn Feature(icon: Asset, title: String, description: String) -> Element {
    rsx! {
        div { class: style::container,
            img { class: style::icon, src: icon }
            h1 { class: style::title, "{title}" }
            span { class: style::description, "{description}" }
        }
    }
}
