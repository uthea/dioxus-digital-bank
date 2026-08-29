use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::asset::{BG_MOBILE, MOCKUPS};

#[with_css(style, "src/component/background_intro.scss")]
#[component]
pub fn BackgroundIntro() -> Element {
    rsx! {
        div { class: style::intro, background_image: "url({BG_MOBILE})",
            img { class: style::image, src: MOCKUPS }
        }
    }
}
