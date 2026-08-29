use dioxus::prelude::*;

#[component]
pub fn Feature(icon: Asset, title: String, description: String) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "24px",
            align_items: "center",
            img {
                height: "72px",
                width: "72px",
                margin_bottom: "calc(40px - 24px)",
                src: icon,
            }
            h1 {
                font_size: "40px",
                text_align: "center",
                font_weight: "300",
                margin: "0",
                "{title}"
            }
            span {
                font_size: "14px",
                text_align: "center",
                color: " hsl(233, 8%, 62%)",
                margin: "0",
                "{description}"
            }
        }
    }
}
