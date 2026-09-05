use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::asset::{BG_DESKTOP, BG_MOBILE, MOCKUPS};

#[with_css(style, "src/component/background_intro.scss")]
#[component]
pub fn BackgroundIntro() -> Element {
    rsx! {
        div {
            class: style::intro,
            style: "--desktop: url({BG_DESKTOP}); --mobile: url({BG_MOBILE})",
            img { class: style::image, src: MOCKUPS }
        }
    }
}
