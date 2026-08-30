use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::component::button::Button;

#[with_css(style, "src/component/invite.scss")]
#[component]
pub fn InviteSection() -> Element {
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
