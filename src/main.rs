use digital_bank::component::{
    article::LatestArticle, background_intro::BackgroundIntro, footer::Footer, header::Header,
    invite::InviteSection, reason::Reason,
};
use dioxus::prelude::*;
use dioxus_style::with_css;

fn main() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("/assets/main.css");

#[with_css(style, "src/main.scss")]
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: style::main,
            Header {}
            BackgroundIntro {}
            InviteSection {}
            Reason {}
            LatestArticle {}
            Footer {}
        }
    }
}
