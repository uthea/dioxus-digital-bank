use dioxus::prelude::*;

use crate::component::button::Button;

#[component]
pub fn InviteSection() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "24px",
            margin_left: "24px",
            margin_right: "24px",
            margin_top: "48px",
            margin_bottom: "56px",
            align_items: "center",
            h1 {
                font_size: "40px",
                text_align: "center",
                font_weight: "300",
                margin: "0",
                "Next generation digital banking"
            }
            span {
                font_size: "14px",
                text_align: "center",
                color: " hsl(233, 8%, 62%)",
                "Take your financial life online. Your Digitalbank account will be a one-stop-shop for spending, saving, budgeting, investing, and much more."
            }
            Button { "Request Invite" }
        }
    }
}
