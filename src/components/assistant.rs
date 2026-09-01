use dioxus::prelude::*;

use crate::data::{match_services, services};
use crate::routes::Route;

#[component]
pub fn Assistant() -> Element {
    let mut question = use_signal(String::new);
    let mut asked = use_signal(|| false);

    let examples = [
        "Perdi minha carteira de motorista",
        "Quero declarar meu imposto de renda",
        "Como tiro passaporte?",
    ];

    // O reconhecimento por palavras-chave em si é uma função pura testável em
    // `crate::data::match_services` — aqui só conectamos o texto digitado (Signal) a ela.
    let matches = use_memo(move || match_services(&question(), &services()));

    rsx! {
        section { id: "assistente", class: "reveal scroll-mt-16 bg-govbr-blue-dark",
            div { class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                div { class: "text-center mb-8",
                    span { class: "inline-flex items-center gap-2 text-xs font-semibold text-govbr-yellow bg-white/10 rounded px-3 py-1 mb-4",
                        "✨ Novidade"
                    }
                    h2 { class: "text-2xl sm:text-3xl font-bold text-white",
                        "Não sabe por onde começar? Pergunte ao assistente"
                    }
                    p { class: "mt-3 text-govbr-blue-soft",
                        "Descreva o que você precisa com suas próprias palavras, sem termos técnicos."
                    }
                }

                form {
                    class: "flex items-center gap-2 bg-white rounded-lg p-2 shadow-lg",
                    onsubmit: move |e| {
                        e.prevent_default();
                        asked.set(true);
                    },
                    input {
                        r#type: "text",
                        placeholder: "Ex: Perdi minha carteira de motorista",
                        class: "flex-1 bg-transparent text-govbr-blue-dark placeholder:text-govbr-gray-text/70 outline-none px-3 py-2.5",
                        value: "{question}",
                        oninput: move |e| {
                            question.set(e.value());
                            asked.set(false);
                        },
                    }
                    button {
                        r#type: "submit",
                        class: "shrink-0 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-5 py-2.5",
                        "Perguntar"
                    }
                }

                div { class: "flex flex-wrap gap-2 mt-3",
                    for ex in examples {
                        button {
                            class: "text-xs text-govbr-blue-soft hover:text-white border border-white/20 hover:border-white/40 rounded-full px-3 py-1.5 transition-colors",
                            onclick: move |_| {
                                question.set(ex.to_string());
                                asked.set(true);
                            },
                            "{ex}"
                        }
                    }
                }

                if asked() {
                    div { class: "mt-6 bg-white rounded-lg p-6",
                        if matches().is_empty() {
                            div {
                                p { class: "text-govbr-blue-dark font-medium",
                                    "Ainda não encontrei um serviço exato para isso."
                                }
                                p { class: "mt-1 text-sm text-govbr-gray-text",
                                    "Tente descrever de outro jeito, ou veja "
                                    a { href: "#servicos", class: "text-govbr-blue underline", "todos os serviços" }
                                    "."
                                }
                            }
                        } else {
                            p { class: "text-govbr-blue-dark font-medium mb-4", "Entendi! Encontrei estes serviços para você:" }
                            div { class: "flex flex-col gap-2",
                                for s in matches() {
                                    Link {
                                        to: Route::ServiceDetail { slug: s.slug.to_string() },
                                        class: "flex items-center gap-3 p-3 rounded border border-govbr-gray-border hover:border-govbr-blue hover:bg-govbr-gray-bg transition-colors",
                                        span { class: "text-2xl", "{s.icon}" }
                                        div { class: "flex-1 min-w-0",
                                            p { class: "text-sm font-semibold text-govbr-blue-dark", "{s.name}" }
                                            p { class: "text-xs text-govbr-gray-text", "{s.tag} · ⏱️ {s.time_estimate}" }
                                        }
                                        span { class: "text-govbr-blue", "→" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
