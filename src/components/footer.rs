use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::ui_state::UiPanels;

/// Rodapé institucional: identifica claramente que este é um projeto de demonstração
/// (HackGOV), não o site oficial gov.br.
///
/// Os links "Central de ajuda" e "Acessibilidade" abrem funcionalidades reais
/// que já existem na aplicação (o `HelpWidget` e o `AccessibilityWidget`).
/// "Fale conosco" abre o cliente de e-mail (`mailto:`) de verdade. Todo o
/// resto ainda não tem página por trás — em vez de um `href="#"` que deixa a
/// URL com uma "#" no final ao passar o mouse (o problema relatado), o clique
/// abre um aviso claro de simulação, coerente com o fato de este ser um
/// projeto de hackathon.
#[component]
pub fn Footer() -> Element {
    let mut ui = use_context::<UiPanels>();

    rsx! {
        footer { id: "ajuda", class: "scroll-mt-16 bg-govbr-blue-dark",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                div { class: "grid sm:grid-cols-2 lg:grid-cols-5 gap-10",
                    div { class: "lg:col-span-1",
                        div { class: "flex items-center gap-2 mb-4",
                            Icon { kind: IconKind::Shield, class: "w-6 h-6 text-govbr-yellow" }
                            span { class: "text-lg font-bold text-white", "hack" }
                            span { class: "text-lg font-bold text-govbr-blue-soft", ".gov" }
                            span { class: "text-lg font-bold text-govbr-yellow", ".br" }
                        }
                        p { class: "text-sm text-govbr-blue-soft leading-relaxed",
                            "Portal digital de serviços, mais tecnológico, seguro e simples para o cidadão."
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", "Sobre o Governo" }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            for label in ["Órgãos do governo", "Acesso à informação", "Dados abertos", "Eixos temáticos", "Painel de monitoramento"] {
                                li {
                                    button {
                                        class: "text-left hover:text-govbr-yellow transition-colors",
                                        onclick: move |_| ui.coming_soon.set(Some(label)),
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", "Ações e Programas" }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            for label in ["Notícias", "Participe", "Legislação", "Ouvidoria (Fala.BR)"] {
                                li {
                                    button {
                                        class: "text-left hover:text-govbr-yellow transition-colors",
                                        onclick: move |_| ui.coming_soon.set(Some(label)),
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", "Ajuda" }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            li {
                                button {
                                    class: "text-left hover:text-govbr-yellow transition-colors",
                                    onclick: move |_| ui.help_open.set(true),
                                    "Central de ajuda"
                                }
                            }
                            li {
                                button {
                                    class: "text-left hover:text-govbr-yellow transition-colors",
                                    onclick: move |_| ui.a11y_open.set(true),
                                    "Acessibilidade"
                                }
                            }
                            li {
                                button {
                                    class: "text-left hover:text-govbr-yellow transition-colors",
                                    onclick: move |_| ui.coming_soon.set(Some("Mapa do site")),
                                    "Mapa do site"
                                }
                            }
                            li {
                                a {
                                    href: "mailto:contato@hack.gov.br",
                                    class: "hover:text-govbr-yellow transition-colors",
                                    "Fale conosco"
                                }
                            }
                            li { span { class: "text-white/50", "Central 0800 000 0000" } }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", "Legal" }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            for label in ["Termos de uso", "Política de privacidade (LGPD)", "Política de cookies"] {
                                li {
                                    button {
                                        class: "text-left hover:text-govbr-yellow transition-colors",
                                        onclick: move |_| ui.coming_soon.set(Some(label)),
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "mt-12 pt-8 border-t border-white/10 flex flex-col sm:flex-row items-center justify-between gap-4",
                    p { class: "text-xs text-govbr-blue-soft", "© 2026 HackGOV. Projeto de demonstração — não é um site oficial do governo." }
                    div { class: "flex items-center gap-4 text-govbr-blue-soft",
                        for (icon , label) in [
                            (IconKind::XLogo, "X (Twitter)"),
                            (IconKind::Facebook, "Facebook"),
                            (IconKind::Instagram, "Instagram"),
                            (IconKind::LinkedIn, "LinkedIn"),
                            (IconKind::YouTube, "YouTube"),
                        ] {
                            button {
                                class: "hover:text-govbr-yellow transition-colors",
                                "aria-label": label,
                                onclick: move |_| ui.coming_soon.set(Some(label)),
                                Icon { kind: icon, class: "w-5 h-5" }
                            }
                        }
                    }
                }
            }
        }
    }
}
