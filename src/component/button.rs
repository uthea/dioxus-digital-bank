use dioxus::prelude::*;

#[component]
pub fn Button(children: Element) -> Element {
    rsx! {
        button {
            background: "linear-gradient(10deg,#2AB6D9,#33D35E)",
            color: "white",
            padding: "16px 32px",
            border_radius: "32px",
            width: "fit-content",
            border: "none",
            font_weight: "700",
            {children}
        }
    }
}
