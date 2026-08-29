use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::asset::{CLOSE, HAMBURGER, LOGO_DARK};

#[with_css(style, "src/component/header.scss")]
#[component]
pub fn Header() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: style::header,
            img { src: LOGO_DARK }
            img {
                src: if open() { CLOSE } else { HAMBURGER },
                onclick: move |_| open.toggle(),
            }
        }

        if open() {
            Menu {}
        }
    }
}

#[with_css(style, "src/component/header.scss")]
#[component]
fn Menu() -> Element {
    rsx! {
        div { class: style::menu_container,
            nav { class: style::menu,
                span { "Home" }
                span { "About" }
                span { "Contact" }
                span { "Blog" }
                span { "Careers" }
            }
        }

        div { class: style::overlay_container,
            div { class: style::overlay }
        }
    }
}
