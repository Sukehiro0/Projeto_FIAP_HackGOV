use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::data::services;
use crate::routes::Route;

/// Seção de busca principal da Home, com resultados em tempo real conforme o usuário digita.
#[component]
pub fn Hero() -> Element {
    let mut query = use_signal(String::new);
    let suggestions = [
        "Imposto de Renda",
        "CPF",
        "Carteira de Trabalho",
        "Passaporte",
        "FGTS",
    ];

    let results = use_memo(move || {
        let q = query().trim().to_lowercase();
        if q.is_empty() {
            return Vec::new();
        }
        services()
            .into_iter()
            .filter(|s| s.name.to_lowercase().contains(&q) || s.tag.to_lowercase().contains(&q))
            .take(5)
            .collect::<Vec<_>>()
    });

    rsx! {
        section { class: "bg-white",
            div { class: "max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 pb-14",
                // Titulo simples estilo gov.br real (sem hero gigante)
                div { class: "flex items-center gap-3 mb-8",
                    Icon { kind: IconKind::Menu, class: "w-6 h-6 text-govbr-blue" }
                    h1 { class: "text-xl sm:text-2xl font-normal text-govbr-gray-text", "Serviços e Informações do Brasil" }
                }

                // Barra de busca
                div { class: "relative",
                    div { class: "flex items-center gap-2 bg-white border border-govbr-gray-border focus-within:border-govbr-blue rounded-lg p-2 shadow-sm transition-colors",
                        input {
                            r#type: "text",
                            placeholder: "O que você procura?",
                            class: "flex-1 bg-transparent text-govbr-blue-dark placeholder:text-govbr-gray-text/70 outline-none px-3 py-2.5",
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                        }
                        button {
                            r#type: "submit",
                            class: "flex items-center justify-center w-10 h-10 rounded-full text-govbr-blue hover:bg-govbr-blue/10 transition-colors",
                            "aria-label": "Buscar",
                            Icon { kind: IconKind::Search, class: "w-5 h-5" }
                        }
                    }

                    // Dropdown de resultados (autocomplete real)
                    if !query().trim().is_empty() {
                        div { class: "absolute left-0 right-0 mt-2 bg-white rounded-lg shadow-xl border border-govbr-gray-border overflow-hidden z-10",
                            if results().is_empty() {
                                p { class: "px-4 py-3 text-sm text-govbr-gray-text",
                                    "Nenhum serviço encontrado para \"{query}\""
                                }
                            } else {
                                for r in results() {
                                    Link {
                                        to: Route::ServiceDetail { slug: r.slug.to_string() },
                                        class: "flex items-center gap-3 px-4 py-3 hover:bg-govbr-gray-bg transition-colors border-b border-govbr-gray-border last:border-0",
                                        Icon { kind: r.icon, class: "w-5 h-5 text-govbr-blue" }
                                        div {
                                            p { class: "text-sm font-semibold text-govbr-blue-dark", "{r.name}" }
                                            p { class: "text-xs text-govbr-gray-text", "{r.tag} · {r.time_estimate}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-wrap items-center gap-2 mt-4",
                    for s in suggestions {
                        button {
                            class: "text-xs font-medium text-govbr-blue bg-govbr-gray-bg hover:bg-govbr-gray-border/60 rounded-full px-3 py-1.5 transition-colors",
                            onclick: move |_| query.set(s.to_string()),
                            "{s}"
                        }
                    }
                }
            }
        }
    }
}
