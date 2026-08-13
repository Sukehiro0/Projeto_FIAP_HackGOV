use dioxus::prelude::*;

use crate::accessibility::speak;
use crate::components::{FeedbackForm, PerformanceCard};
use crate::data::services;
use crate::routes::Route;

#[derive(Clone, Copy, PartialEq)]
enum FaceCheck {
    Idle,
    Failed,
    Resolved,
}

const PROCESS_STEPS: [&str; 6] = [
    "Solicitação enviada",
    "Documentos analisados",
    "Em processamento",
    "Aguardando órgão",
    "Aprovado",
    "Concluído",
];

#[component]
pub fn ServiceDetail(slug: String) -> Element {
    let service = services().into_iter().find(|s| s.slug == slug);
    let mut confirmed = use_signal(|| false);
    let mut started = use_signal(|| false);
    let mut face_check = use_signal(|| FaceCheck::Idle);
    let mut step = use_signal(|| 0usize);
    let mut simple_lang = use_signal(|| false);

    rsx! {
        div { class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            if let Some(s) = service {
                Link {
                    to: Route::Home {},
                    class: "text-sm font-medium text-govbr-blue hover:underline",
                    "← Voltar para a página inicial"
                }
                div { class: "mt-6 flex items-start gap-4",
                    span { class: "text-4xl", "{s.icon}" }
                    div {
                        span { class: "text-xs font-semibold text-govbr-blue uppercase tracking-wide", "{s.tag}" }
                        h1 { class: "text-3xl font-bold text-govbr-blue-dark", "{s.name}" }
                    }
                }
                p { class: "mt-4 text-govbr-gray-text leading-relaxed",
                    if simple_lang() { "{s.simple_explanation}" } else { "{s.description}" }
                }
                div { class: "mt-3 flex flex-wrap items-center gap-3",
                    button {
                        class: "text-xs font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-3 py-1.5",
                        onclick: move |_| simple_lang.toggle(),
                        if simple_lang() { "Ver texto oficial" } else { "🧠 Linguagem simples" }
                    }
                    button {
                        class: "text-xs font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-3 py-1.5",
                        onclick: move |_| {
                            let text = if simple_lang() { s.simple_explanation } else { s.description };
                            speak(text);
                        },
                        "🔊 Ouvir"
                    }
                }
                div { class: "mt-4 inline-flex items-center gap-2 text-sm font-medium text-govbr-green bg-govbr-green/10 rounded px-4 py-2",
                    "⏱️ Tempo estimado: {s.time_estimate}"
                }
                PerformanceCard { service: s.clone() }

                if !started() {
                    // Diagnóstico prévio: evita que o cidadão descubra no meio do processo que falta algo.
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark mb-1", "Antes de começar" }
                        p { class: "text-sm text-govbr-gray-text mb-4",
                            "Confira se você tem tudo isso pronto, para não ter surpresas no meio do caminho:"
                        }
                        ul { class: "flex flex-col gap-2 mb-4",
                            for req in s.requirements.iter().copied() {
                                li { class: "flex items-center gap-2 text-sm text-govbr-gray-text",
                                    span { class: "text-govbr-green", "✓" }
                                    "{req}"
                                }
                            }
                        }
                        label { class: "flex items-center gap-2 text-sm font-medium text-govbr-blue-dark cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: confirmed(),
                                onchange: move |e| confirmed.set(e.checked()),
                            }
                            "Confirmo que tenho os itens acima"
                        }
                    }

                    button {
                        class: if confirmed() { "mt-6 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-6 py-3" } else { "mt-6 text-sm font-semibold text-white bg-govbr-gray-border cursor-not-allowed rounded-full px-6 py-3" },
                        disabled: !confirmed(),
                        onclick: move |_| {
                            started.set(true);
                            if !s.needs_biometrics {
                                step.set(0);
                            }
                        },
                        "Iniciar: {s.name}"
                    }
                } else if s.needs_biometrics && face_check() != FaceCheck::Resolved {
                    // Simulador de verificação facial com fallback (dor real relatada por usuários do gov.br)
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6 text-center",
                        if face_check() == FaceCheck::Idle {
                            span { class: "text-4xl", "🤳" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "Verificação facial necessária" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "Precisamos confirmar sua identidade antes de continuar." }
                            button {
                                class: "mt-4 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-6 py-2.5",
                                onclick: move |_| face_check.set(FaceCheck::Failed),
                                "Simular verificação facial"
                            }
                        } else {
                            span { class: "text-4xl", "⚠️" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "O reconhecimento facial não funcionou" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "Sem problemas. Vamos tentar outra forma de confirmar sua identidade:" }
                            div { class: "flex flex-wrap justify-center gap-2 mt-4",
                                for alt in ["🏦 Banco credenciado", "📧 E-mail", "📞 Telefone", "🏢 Atendimento presencial"] {
                                    button {
                                        class: "text-sm font-medium text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                        onclick: move |_| {
                                            face_check.set(FaceCheck::Resolved);
                                            step.set(0);
                                        },
                                        "{alt}"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Acompanhamento visual do processo (reduz a ansiedade do "será que deu certo?")
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark mb-5", "Acompanhamento da solicitação" }
                        div { class: "flex flex-col",
                            for (i , label) in PROCESS_STEPS.iter().enumerate() {
                                div { class: "flex items-start gap-3",
                                    div { class: "flex flex-col items-center",
                                        span {
                                            class: if i <= step() { "flex items-center justify-center w-7 h-7 rounded-full bg-govbr-green text-white text-xs font-bold shrink-0" } else { "flex items-center justify-center w-7 h-7 rounded-full bg-govbr-gray-border text-govbr-gray-text text-xs font-bold shrink-0" },
                                            if i <= step() { "✓" } else { "{i + 1}" }
                                        }
                                        if i < PROCESS_STEPS.len() - 1 {
                                            div { class: if i < step() { "w-px flex-1 min-h-6 bg-govbr-green" } else { "w-px flex-1 min-h-6 bg-govbr-gray-border" } }
                                        }
                                    }
                                    p {
                                        class: if i <= step() { "text-sm font-medium text-govbr-blue-dark pb-6" } else { "text-sm text-govbr-gray-text pb-6" },
                                        "{label}"
                                    }
                                }
                            }
                        }
                        if step() < PROCESS_STEPS.len() - 1 {
                            button {
                                class: "text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-5 py-2",
                                onclick: move |_| step.set((step() + 1).min(PROCESS_STEPS.len() - 1)),
                                "Simular avanço da etapa"
                            }
                        } else {
                            p { class: "text-sm font-semibold text-govbr-green", "✓ Processo concluído com sucesso!" }
                            FeedbackForm {}
                        }
                    }
                }
            } else {
                div { class: "text-center",
                    h1 { class: "text-2xl font-bold text-govbr-blue-dark", "Serviço não encontrado" }
                    p { class: "mt-2 text-govbr-gray-text", "O serviço que você procura não existe ou foi removido." }
                    Link {
                        to: Route::Home {},
                        class: "mt-4 inline-block text-govbr-blue hover:underline",
                        "← Voltar para a página inicial"
                    }
                }
            }
        }
    }
}
