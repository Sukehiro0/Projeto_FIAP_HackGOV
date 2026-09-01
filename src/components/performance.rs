use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::data::Service;

/// Transparência de desempenho: nota, tempo médio e principais problemas de cada serviço,
/// no espírito de "Google Maps dos serviços públicos".
#[component]
pub fn PerformanceCard(service: Service) -> Element {
    let full_stars = service.rating.round() as i32;

    rsx! {
        div { class: "mt-8 bg-govbr-gray-bg rounded-lg p-6",
            h2 { class: "text-base font-semibold text-govbr-blue-dark mb-4", "Transparência de desempenho" }
            div { class: "flex items-center gap-2 mb-4",
                div { class: "flex items-center gap-0.5",
                    for i in 0..5 {
                        if i < full_stars {
                            Icon { kind: IconKind::Star, class: "w-4 h-4 text-govbr-yellow" }
                        } else {
                            Icon { kind: IconKind::StarOutline, class: "w-4 h-4 text-govbr-gray-border" }
                        }
                    }
                }
                span { class: "text-sm font-semibold text-govbr-blue-dark", "{service.rating}" }
                span { class: "text-xs text-govbr-gray-text", "({service.complaints} avaliações)" }
            }
            div { class: "grid grid-cols-2 sm:grid-cols-3 gap-4",
                div {
                    p { class: "text-xl font-bold text-govbr-blue-dark", "{service.avg_days} dias" }
                    p { class: "text-xs text-govbr-gray-text", "Tempo médio" }
                }
                div {
                    p { class: "text-xl font-bold text-govbr-green", "{service.resolved_pct}%" }
                    p { class: "text-xs text-govbr-gray-text", "Dentro do prazo" }
                }
                div {
                    p { class: "text-xl font-bold text-govbr-blue-dark", "{service.complaints}" }
                    p { class: "text-xs text-govbr-gray-text", "Reclamações" }
                }
            }
            p { class: "mt-4 text-xs text-govbr-gray-text",
                "Principal problema relatado: "
                span { class: "font-medium text-govbr-blue-dark", "{service.main_issue}" }
            }
        }
    }
}
