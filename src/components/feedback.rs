use dioxus::prelude::*;

const ISSUES: [&str; 7] = [
    "Demorou mais do que esperado",
    "Não entendi alguma etapa",
    "O sistema apresentou erro",
    "Pediu documento desnecessário",
    "Não consegui acessar",
    "Alguma informação estava errada",
    "Não resolveu meu problema",
];

/// Feedback detalhado (não só estrelas) ao final de um processo concluído.
#[component]
pub fn FeedbackForm() -> Element {
    let mut sent = use_signal(|| false);
    let mut rating = use_signal(|| 0i32);
    let mut checked = use_signal(|| vec![false; ISSUES.len()]);

    rsx! {
        div { class: "mt-6 border-t border-govbr-gray-border pt-6",
            if sent() {
                div { class: "text-center py-4",
                    span { class: "text-3xl", "🙏" }
                    p { class: "mt-2 text-sm font-semibold text-govbr-blue-dark", "Obrigado pelo feedback!" }
                    p { class: "text-xs text-govbr-gray-text mt-1",
                        "Suas respostas ajudam a melhorar este serviço para todos."
                    }
                }
            } else {
                h3 { class: "text-sm font-semibold text-govbr-blue-dark mb-3", "Como foi sua experiência com este serviço?" }
                div { class: "flex gap-1 mb-4",
                    for i in 1..=5 {
                        button {
                            class: if i <= rating() { "text-2xl text-govbr-yellow" } else { "text-2xl text-govbr-gray-border" },
                            onclick: move |_| rating.set(i),
                            "★"
                        }
                    }
                }
                p { class: "text-xs text-govbr-gray-text mb-2", "O que poderia ser melhor? (opcional)" }
                div { class: "flex flex-col gap-2 mb-4",
                    for (i , issue) in ISSUES.iter().enumerate() {
                        label { class: "flex items-center gap-2 text-sm text-govbr-gray-text cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: checked()[i],
                                onchange: move |e| {
                                    let mut list = checked();
                                    list[i] = e.checked();
                                    checked.set(list);
                                },
                            }
                            "{issue}"
                        }
                    }
                }
                button {
                    class: if rating() > 0 { "text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-5 py-2.5" } else { "text-sm font-semibold text-white bg-govbr-gray-border cursor-not-allowed rounded-full px-5 py-2.5" },
                    disabled: rating() == 0,
                    onclick: move |_| sent.set(true),
                    "Enviar feedback"
                }
            }
        }
    }
}
