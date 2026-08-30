use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::asset::{AIR_PLANE, CONFETTI, CURRENCY, RESTAURANT};

#[with_css(style, "src/component/article.scss")]
#[component]
pub fn LatestArticle() -> Element {
    rsx! {
        div { class: style::latest_article,
            h1 { class: style::latest_article_title, "Latest Article" }
            div { class: style::latest_article_container,
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

#[with_css(style, "src/component/article.scss")]
#[component]
fn Article(thumbnail: Asset, author: String, title: String, summary: String) -> Element {
    rsx! {
        div { class: style::article,
            img { class: style::article_thumbnail, src: thumbnail }
            div { class: style::article_content,
                span { class: style::article_content_author, "By {author}" }

                div { class: style::article_content_area,
                    h1 { class: style::article_content_title, "{title}" }
                    span { class: style::article_content_summary, "{summary} ..." }
                }

            }
        }
    }
}
