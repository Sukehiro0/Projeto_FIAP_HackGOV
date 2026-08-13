use dioxus::prelude::*;

use crate::data::categories;

#[component]
pub fn Categories() -> Element {
    rsx! {
        section { id: "categorias", class: "reveal scroll-mt-16 bg-white border-y border-govbr-gray-border",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                h2 { class: "text-2xl sm:text-3xl font-bold text-govbr-blue-dark mb-2 text-center", "Navegue por categoria" }
                p { class: "text-govbr-gray-text mb-10 text-center", "Encontre o serviço ou a informação que você precisa" }

                div { class: "grid sm:grid-cols-2 lg:grid-cols-3 gap-4",
                    for category in categories() {
                        a {
                            href: "#",
                            class: "flex flex-col items-center justify-center gap-3 text-center bg-govbr-gray-bg hover:bg-govbr-gray-border/50 rounded p-8 transition-colors",
                            span { class: "text-4xl", "{category.icon}" }
                            span { class: "text-sm font-bold uppercase tracking-wide text-govbr-blue-dark", "{category.name}" }
                        }
                    }
                }
            }
        }
    }
}
