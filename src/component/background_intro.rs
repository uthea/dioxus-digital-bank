use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::asset::{BG_DESKTOP, BG_MOBILE, MOCKUPS};
use crate::component::button::Button;

#[with_css(style, "src/component/background_intro.scss")]
#[component]
pub fn BackgroundIntro() -> Element {
    rsx! {
        div { class: style::container,
            div {
                class: style::intro,
                style: "--desktop: url({BG_DESKTOP}); --mobile: url({BG_MOBILE})",
                img { class: style::image, src: MOCKUPS }
            }
            InviteSection {}
        }

    }
}

#[with_css(style, "src/component/invite.scss")]
#[component]
fn InviteSection() -> Element {
    rsx! {
        div { class: style::container,
            h1 { class: style::title, "Next generation digital banking" }
            span { class: style::description,
                "Take your financial life online. Your Digitalbank account will be a one-stop-shop for spending, saving, budgeting, investing, and much more."
            }
            Button { "Request Invite" }
        }
    }
}
