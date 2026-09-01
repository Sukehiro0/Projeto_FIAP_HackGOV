use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::data::services;
use crate::routes::Route;

/// Lista numerada dos serviços mais buscados, com tempo estimado de cada um.
#[component]
pub fn Services() -> Element {
    rsx! {
        section { id: "servicos", class: "reveal scroll-mt-16 max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            div { class: "text-center mb-10",
                h2 { class: "text-2xl sm:text-3xl font-bold text-govbr-blue-dark", "Serviços para você" }
                p { class: "mt-2 text-govbr-gray-text", "Os serviços que os cidadãos mais usam no portal" }
            }

            div { class: "grid sm:grid-cols-2 gap-x-12 border-t border-govbr-gray-border",
                for (i , service) in services().into_iter().enumerate() {
                    Link {
                        to: Route::ServiceDetail { slug: service.slug.to_string() },
                        class: "group flex items-center gap-4 py-4 border-b border-govbr-gray-border",
                        span { class: "text-lg font-light text-govbr-gray-text w-8 shrink-0", "{i + 1}" }
                        Icon { kind: service.icon, class: "w-6 h-6 text-govbr-blue shrink-0" }
                        div { class: "flex-1 min-w-0",
                            p { class: "text-xs text-govbr-gray-text", "{service.tag}" }
                            p { class: "text-govbr-blue font-semibold group-hover:underline", "{service.name}" }
                        }
                        span { class: "flex items-center gap-1 text-xs text-govbr-gray-text shrink-0",
                            Icon { kind: IconKind::Clock, class: "w-3.5 h-3.5" }
                            "{service.time_estimate}"
                        }
                    }
                }
            }
        }
    }
}
