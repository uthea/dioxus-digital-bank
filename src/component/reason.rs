use dioxus::prelude::*;
use dioxus_style::with_css;

use crate::{
    asset::{API, BUDGETING, ONBOARDING, ONLINE},
    component::feature::Feature,
};

#[with_css(style, "src/component/reason.scss")]
#[component]
pub fn Reason() -> Element {
    rsx! {
        div { class: style::container,
            div { class: style::reason,
                h1 { class: style::title, "Why choose Digitalbank?" }

                span { class: style::description,
                    "We leverage Open Banking to turn your bank account into your financial hub. Control your finances like never before."
                }
            }

            div { class: style::feature_container,
                Feature {
                    icon: ONLINE,
                    title: "Online Banking",
                    description: "Our modern web and mobile applications allow you to keep track of your finances wherever you are in the world.",
                }
                Feature {
                    icon: BUDGETING,
                    title: "Simple Budgeting",
                    description: "See exactly where your money goes each month. Receive notifications when you’re close to hitting your limits.",
                }
                Feature {
                    icon: ONBOARDING,
                    title: "Fast Onboarding",
                    description: "We don’t do branches. Open your account in minutes online and start taking control of your finances right away.",
                }
                Feature {
                    icon: API,
                    title: "Open API",
                    description: "Manage your savings, investments, pension, and much more from one account. Tracking your money has never been easier.",
                }
            }
        }
    }
}
