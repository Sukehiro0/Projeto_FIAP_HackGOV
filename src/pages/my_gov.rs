use dioxus::prelude::*;

use crate::components::FraudShield;
use crate::data::{alerts, life_areas, LifeState};
use crate::routes::Route;

/// Painel "Minha vida no governo": visão única centrada no cidadão, em vez de
/// espalhada pela estrutura dos órgãos.
#[component]
pub fn MyGovPanel() -> Element {
    let areas = life_areas();
    let notices = alerts();

    rsx! {
        div { class: "max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            Link {
                to: Route::Home {},
                class: "text-sm font-medium text-govbr-blue hover:underline",
                "← Voltar para a página inicial"
            }
            h1 { class: "mt-6 text-3xl font-bold text-govbr-blue-dark", "Minha vida no governo" }
            p { class: "mt-2 text-govbr-gray-text",
                "Um painel único com tudo que envolve você, em vez de espalhado por dezenas de sistemas diferentes."
            }

            if !notices.is_empty() {
                div { class: "mt-8 bg-govbr-blue-dark rounded-lg p-5",
                    h2 { class: "text-sm font-semibold text-white mb-3", "🔔 Requer sua atenção" }
                    div { class: "flex flex-col gap-2",
                        for n in notices.iter() {
                            div { class: "flex items-start gap-3 bg-white/10 rounded-lg p-3",
                                span { class: "text-xl", "{n.icon}" }
                                div {
                                    p { class: "text-sm font-medium text-white", "{n.title}" }
                                    p { class: "text-xs text-govbr-blue-soft mt-0.5", "{n.message}" }
                                }
                            }
                        }
                    }
                }
            }

            FraudShield {}

            div { class: "mt-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4",
                for area in areas.iter() {
                    div { class: "bg-govbr-gray-bg rounded-lg p-5",
                        div { class: "flex items-center gap-3 mb-2",
                            span { class: "text-2xl", "{area.icon}" }
                            h3 { class: "text-sm font-semibold text-govbr-blue-dark", "{area.name}" }
                        }
                        p {
                            class: match area.state {
                                LifeState::Ok => "text-sm text-govbr-gray-text",
                                LifeState::Attention => "text-sm font-medium text-amber-700",
                                LifeState::Pending => "text-sm font-medium text-red-600",
                            },
                            "{area.status}"
                        }
                        if let Some(slug) = area.related_slug {
                            Link {
                                to: Route::ServiceDetail { slug: slug.to_string() },
                                class: "inline-block mt-3 text-xs font-semibold text-govbr-blue hover:underline",
                                "Resolver agora →"
                            }
                        }
                    }
                }
            }
        }
    }
}
