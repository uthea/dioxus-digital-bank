use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::{
    asset::{FACEBOOK, INSTAGRAM, LOGO_LIGHT, PINTEREST, TWITTER, YOUTUBE},
    component::button::Button,
};

#[with_css(style, "src/component/footer.scss")]
#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: style::container,
            img { src: LOGO_LIGHT }

            div { class: style::social_media,
                img { src: FACEBOOK }
                img { src: YOUTUBE }
                img { src: TWITTER }
                img { src: PINTEREST }
                img { src: INSTAGRAM }
            }

            nav { class: style::navigation,
                span { font_size: "14px", color: "white", "About Us" }
                span { font_size: "14px", color: "white", "Contact" }
                span { font_size: "14px", color: "white", "Blog" }
                span { font_size: "14px", color: "white", "Careers" }
                span { font_size: "14px", color: "white", "Support" }
                span { font_size: "14px", color: "white", "Privacy Policy" }
            }

            div { class: style::trademark_container,
                Button { "Request Invite" }
                span { class: style::trademark, "© Digitalbank. All Rights Reserved" }
            }
        }
    }
}
