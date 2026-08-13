use dioxus::prelude::*;

use crate::data::{attendance_points, help_reasons};

/// Botão flutuante "Não consigo resolver": em qualquer página, oferece um menu de
/// motivos de bloqueio comuns e a solução direta para cada um (dor real relatada
/// pelos usuários do gov.br: não saber a quem recorrer quando algo dá errado).
#[component]
pub fn HelpWidget() -> Element {
    let mut open = use_signal(|| false);
    let mut selected = use_signal(|| None::<usize>);
    let reasons = help_reasons();
    let points = attendance_points();

    rsx! {
        button {
            class: "fixed bottom-6 right-6 z-40 flex items-center gap-2 text-sm font-semibold text-white bg-govbr-blue-dark hover:bg-govbr-blue transition-colors rounded-full pl-4 pr-5 py-3 shadow-lg",
            onclick: move |_| {
                open.set(true);
                selected.set(None);
            },
            span { "🆘" }
            span { class: "hidden sm:inline", "Não consigo resolver" }
        }

        if open() {
            div {
                class: "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40 p-4",
                onclick: move |_| open.set(false),
                div {
                    class: "bg-white rounded-lg shadow-2xl w-full max-w-md max-h-[85vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "px-6 py-4 border-b border-govbr-gray-border flex items-center justify-between",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark", "Não conseguiu concluir?" }
                        button {
                            class: "text-govbr-gray-text hover:text-govbr-blue-dark text-lg",
                            onclick: move |_| open.set(false),
                            "✕"
                        }
                    }
                    div { class: "p-6",
                        if let Some(i) = selected() {
                            if let Some(reason) = reasons.get(i) {
                                button {
                                    class: "text-sm font-medium text-govbr-blue hover:underline mb-4",
                                    onclick: move |_| selected.set(None),
                                    "← Voltar"
                                }
                                div { class: "flex items-start gap-3 mb-4",
                                    span { class: "text-2xl", "{reason.icon}" }
                                    h3 { class: "text-sm font-semibold text-govbr-blue-dark pt-1", "{reason.label}" }
                                }
                                p { class: "text-sm text-govbr-gray-text leading-relaxed bg-govbr-gray-bg rounded-lg p-4",
                                    "{reason.solution}"
                                }

                                div { class: "mt-6",
                                    h4 { class: "text-xs font-semibold text-govbr-blue-dark uppercase tracking-wide mb-2",
                                        "Prefere resolver pessoalmente?"
                                    }
                                    div { class: "flex flex-col gap-2",
                                        for p in points.iter().take(2) {
                                            div { class: "border border-govbr-gray-border rounded-lg p-3",
                                                p { class: "text-sm font-medium text-govbr-blue-dark", "{p.name}" }
                                                p { class: "text-xs text-govbr-gray-text mt-0.5", "{p.address} · {p.distance_km} km" }
                                                p { class: "text-xs text-govbr-gray-text", "{p.hours}" }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            p { class: "text-sm text-govbr-gray-text mb-4", "Onde você está tendo problema?" }
                            div { class: "flex flex-col gap-2",
                                for (i , reason) in reasons.iter().enumerate() {
                                    button {
                                        class: "flex items-center gap-3 text-left text-sm font-medium text-govbr-blue-dark border border-govbr-gray-border hover:border-govbr-blue hover:bg-govbr-gray-bg transition-colors rounded-lg px-4 py-3",
                                        onclick: move |_| selected.set(Some(i)),
                                        span { class: "text-xl", "{reason.icon}" }
                                        "{reason.label}"
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
