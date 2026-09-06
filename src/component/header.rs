use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::{
    asset::{CLOSE, HAMBURGER, LOGO_DARK},
    component::button::Button,
};

#[with_css(style, "src/component/header.scss")]
#[component]
pub fn Header() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: style::header,
            img { src: LOGO_DARK }
            img {
                class: style::hamburger,
                src: if open() { CLOSE } else { HAMBURGER },
                onclick: move |_| open.toggle(),
            }
            div { class: style::desktop_only, Menu {} }
            div { class: style::desktop_only,
                Button { "Request Invite" }
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
                div { class: style::menu_wrapper,
                    span { class: style::menu_item, "Home" }
                    div { class: style::menu_line }
                }
                div { class: style::menu_wrapper,
                    span { class: style::menu_item, "About" }
                    div { class: style::menu_line }
                }
                div { class: style::menu_wrapper,
                    span { class: style::menu_item, "Contact" }
                    div { class: style::menu_line }
                }
                div { class: style::menu_wrapper,
                    span { class: style::menu_item, "Blog" }
                    div { class: style::menu_line }
                }
                div { class: style::menu_wrapper,
                    span { class: style::menu_item, "Careers" }
                    div { class: style::menu_line }
                }
            }
        }

        div { class: style::overlay_container,
            div { class: style::overlay }
        }
    }
}
