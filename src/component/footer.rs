use dioxus::prelude::*;

use crate::{
    asset::{FACEBOOK, INSTAGRAM, LOGO_LIGHT, PINTEREST, TWITTER, YOUTUBE},
    component::button::Button,
};

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            padding: "54.82px 75.5px",
            display: "flex",
            flex_direction: "column",
            gap: "32px",
            background_color: "hsl(233, 26%, 24%)",
            align_items: "center",

            img { src: LOGO_LIGHT }

            div { display: "flex", gap: "16px",
                img { src: FACEBOOK }
                img { src: YOUTUBE }
                img { src: TWITTER }
                img { src: PINTEREST }
                img { src: INSTAGRAM }
            }

            div {
                display: "flex",
                flex_direction: "column",
                gap: "16px",
                align_items: "center",
                span { font_size: "14px", color: "white", "About Us" }
                span { font_size: "14px", color: "white", "Contact" }
                span { font_size: "14px", color: "white", "Careers" }
                span { font_size: "14px", color: "white", "Support" }
                span { font_size: "14px", color: "white", "Privacy Policy" }
            }

            div {
                display: "flex",
                flex_direction: "column",
                gap: "24px",
                align_items: "center",
                Button { "Request Invite" }
                span { font_size: "14px", color: "hsl(233, 8%, 62%)",
                    "© Digitalbank. All Rights Reserved"
                }
            }
        }
    }
}
