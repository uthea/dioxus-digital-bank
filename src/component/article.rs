use dioxus::prelude::*;

use crate::asset::{AIR_PLANE, CONFETTI, CURRENCY, RESTAURANT};

#[component]
pub fn LatestArticle() -> Element {
    rsx! {
        div {
            padding: "64px 24px",
            display: "flex",
            flex_direction: "column",
            gap: "48px",
            background_color: "hsl(0, 0%, 98%)",

            h1 { font_size: "40px", text_align: "center", font_weight: "300", "Latest Article" }

            div { display: "flex", flex_direction: "column", gap: "16px",
                Article {
                    thumbnail: CURRENCY,
                    author: "Claire Robinson",
                    title: "Receive money in any currency with no fees",
                    summary: "The world is getting smaller and we’re becoming more mobile. So why should you be forced to only receive money in a single",
                }
                Article {
                    thumbnail: RESTAURANT,
                    author: "Wilson Hutton",
                    title: "Treat yourself without worrying about money",
                    summary: "Our simple budgeting feature allows you to separate out your spending and set realistic limits each month. That means you",
                }
                Article {
                    thumbnail: AIR_PLANE,
                    author: "Wilson Hutton",
                    title: "Take your Digitalbank card wherever you go",
                    summary: "We want you to enjoy your travels. This is why we don’t charge any fees on purchases while you’re abroad. We’ll even",
                }
                Article {
                    thumbnail: CONFETTI,
                    author: "Claire Robinson",
                    title: "Our invite-only Beta accounts are now live!",
                    summary: "After a lot of hard work by the whole team, we’re excited to launch our closed beta. It’s easy to request an invite through",
                }
            }
        }
    }
}

#[component]
fn Article(thumbnail: Asset, author: String, title: String, summary: String) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "24px",
            background_color: "white",

            img {
                border_top_left_radius: "5px",
                border_top_right_radius: "5px",
                src: thumbnail,
            }
            div {
                margin_left: "31.5px",
                margin_right: "31.5px",
                margin_bottom: "29px",
                display: "flex",
                flex_direction: "column",
                gap: "16px",

                span {
                    font_size: "10px",
                    color: " hsl(233, 8%, 62%)",
                    margin: "0",
                    "By {author}"
                }

                div { display: "flex", flex_direction: "column", gap: "8px",
                    h1 { font_size: "16px", font_weight: "300", margin: "0", "{title}" }
                    span {
                        font_size: "14px",
                        color: " hsl(233, 8%, 62%)",
                        margin: "0",
                        "{summary} ..."
                    }
                }

            }
        }
    }
}
