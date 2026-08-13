use dioxus::prelude::*;

use crate::data::{alerts, AlertLevel};

#[component]
pub fn AlertsBell() -> Element {
    let mut open = use_signal(|| false);
    let items = alerts();
    let count = items.len();

    rsx! {
        div { class: "relative",
            button {
                class: "relative flex items-center justify-center w-9 h-9 rounded-full hover:bg-govbr-gray-bg transition-colors text-lg",
                title: "Alertas",
                "aria-label": "Ver alertas",
                onclick: move |_| open.toggle(),
                "🔔"
                if count > 0 {
                    span { class: "absolute -top-0.5 -right-0.5 flex items-center justify-center w-4 h-4 rounded-full bg-govbr-yellow text-govbr-blue-dark text-[10px] font-bold",
                        "{count}"
                    }
                }
            }
            if open() {
                div { class: "absolute right-0 mt-2 w-80 max-w-[90vw] bg-white border border-govbr-gray-border rounded-lg shadow-xl z-50 overflow-hidden",
                    div { class: "px-4 py-3 border-b border-govbr-gray-border flex items-center justify-between",
                        h3 { class: "text-sm font-semibold text-govbr-blue-dark", "Seus alertas" }
                        button {
                            class: "text-xs text-govbr-gray-text hover:text-govbr-blue-dark",
                            onclick: move |_| open.set(false),
                            "Fechar"
                        }
                    }
                    div { class: "max-h-96 overflow-y-auto",
                        for a in items.iter() {
                            div { class: "flex items-start gap-3 px-4 py-3 border-b border-govbr-gray-border last:border-0 hover:bg-govbr-gray-bg",
                                span {
                                    class: match a.level {
                                        AlertLevel::Urgent => "shrink-0 w-2 h-2 rounded-full bg-red-500 mt-2",
                                        AlertLevel::Warning => "shrink-0 w-2 h-2 rounded-full bg-govbr-yellow mt-2",
                                        AlertLevel::Info => "shrink-0 w-2 h-2 rounded-full bg-govbr-blue mt-2",
                                    },
                                }
                                span { class: "text-xl shrink-0", "{a.icon}" }
                                div {
                                    p { class: "text-sm font-medium text-govbr-blue-dark", "{a.title}" }
                                    p { class: "text-xs text-govbr-gray-text mt-0.5", "{a.message}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
