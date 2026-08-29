use dioxus::prelude::*;

use crate::{
    asset::{API, BUDGETING, ONBOARDING, ONLINE},
    component::feature::Feature,
};

#[component]
pub fn Reason() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "48px",
            padding: "64px 24px",
            background_color: "#f4f5f7",
            div { display: "flex", flex_direction: "column", gap: "16px",
                h1 {
                    margin: "0",
                    font_size: "40px",
                    text_align: "center",
                    font_weight: "300",
                    "Why choose Digitalbank?"
                }

                span {
                    font_size: "14px",
                    text_align: "center",
                    color: " hsl(233, 8%, 62%)",
                    "We leverage Open Banking to turn your bank account into your financial hub. Control your finances like never before."
                }
            }

            div { display: "flex", flex_direction: "column", gap: "32px",
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
