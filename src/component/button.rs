use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css(style, "src/component/button.scss")]
#[component]
pub fn Button(children: Element) -> Element {
    rsx! {
        button { class: style::button, {children} }
    }
}
