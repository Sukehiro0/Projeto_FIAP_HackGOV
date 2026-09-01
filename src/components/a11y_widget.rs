use dioxus::prelude::*;

use crate::accessibility::A11ySettings;
use crate::components::{Icon, IconKind};
use crate::ui_state::UiPanels;

/// Botão flutuante de acessibilidade, sempre visível em qualquer página e em
/// qualquer tamanho de tela.
///
/// Antes, os controles de acessibilidade (tamanho de fonte, alto contraste,
/// modo fácil) ficavam como texto pequeno em uma barra de 36px de altura,
/// com "Alto contraste" e "Modo fácil" escondidos em telas pequenas
/// (`hidden sm:inline`) — justamente o público que mais depende dessas
/// opções (quem acessa pelo celular) não conseguia vê-las. Um recurso de
/// acessibilidade que não é visto não é usado. Este widget corrige isso:
/// fica fixo na tela, com ícone + texto, alvo de toque de 44px (mínimo
/// recomendado pela WCAG 2.5.5) e visível em toda a jornada do usuário.
///
/// O estado de aberto/fechado mora em `UiPanels` (contexto compartilhado),
/// não num `use_signal` local, porque o link "Acessibilidade" no rodapé
/// também precisa conseguir abrir este painel.
#[component]
pub fn AccessibilityWidget() -> Element {
    let ui = use_context::<UiPanels>();
    let mut open = ui.a11y_open;
    let mut a11y = use_context::<A11ySettings>();
    let font_scale = (a11y.font_scale)();
    let high_contrast = (a11y.high_contrast)();
    let easy_mode = (a11y.easy_mode)();

    rsx! {
        button {
            class: "fixed bottom-6 left-6 z-40 flex items-center justify-center w-14 h-14 rounded-full text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors shadow-lg",
            "aria-label": "Abrir opções de acessibilidade",
            "aria-expanded": if open() { "true" } else { "false" },
            onclick: move |_| open.toggle(),
            Icon { kind: IconKind::Accessibility, class: "w-7 h-7" }
        }

        if open() {
            div {
                class: "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40 p-4",
                onclick: move |_| open.set(false),
                div {
                    class: "bg-white rounded-lg shadow-2xl w-full max-w-sm max-h-[85vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    role: "dialog",
                    "aria-label": "Opções de acessibilidade",
                    div { class: "px-6 py-4 border-b border-govbr-gray-border flex items-center justify-between",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark flex items-center gap-2",
                            Icon { kind: IconKind::Accessibility, class: "w-5 h-5 text-govbr-blue" }
                            "Acessibilidade"
                        }
                        button {
                            class: "flex items-center justify-center w-9 h-9 text-govbr-gray-text hover:text-govbr-blue-dark",
                            "aria-label": "Fechar",
                            onclick: move |_| open.set(false),
                            Icon { kind: IconKind::Close, class: "w-5 h-5" }
                        }
                    }
                    div { class: "p-6 flex flex-col gap-6",
                        div {
                            h3 { class: "text-sm font-semibold text-govbr-blue-dark mb-3", "Tamanho do texto" }
                            div { class: "flex items-center gap-2",
                                button {
                                    class: "flex-1 min-h-[44px] flex items-center justify-center gap-1 text-sm font-semibold text-govbr-blue border border-govbr-blue rounded-lg hover:bg-govbr-blue/5",
                                    "aria-label": "Diminuir tamanho do texto",
                                    onclick: move |_| a11y.decrease_font(),
                                    "A-"
                                }
                                button {
                                    class: "flex-1 min-h-[44px] flex items-center justify-center gap-1 text-sm font-semibold text-govbr-blue border border-govbr-blue rounded-lg hover:bg-govbr-blue/5",
                                    "aria-label": "Restaurar tamanho padrão do texto",
                                    onclick: move |_| a11y.reset_font(),
                                    "Padrão"
                                }
                                button {
                                    class: "flex-1 min-h-[44px] flex items-center justify-center gap-1 text-sm font-semibold text-govbr-blue border border-govbr-blue rounded-lg hover:bg-govbr-blue/5",
                                    "aria-label": "Aumentar tamanho do texto",
                                    onclick: move |_| a11y.increase_font(),
                                    "A+"
                                }
                            }
                            p { class: "mt-2 text-xs text-govbr-gray-text",
                                if font_scale == 0 { "Tamanho padrão" } else if font_scale > 0 { "Aumentado" } else { "Reduzido" }
                            }
                        }

                        button {
                            class: if high_contrast { "flex items-center gap-3 min-h-[44px] px-4 py-3 rounded-lg border-2 border-govbr-blue bg-govbr-blue/5 text-left" } else { "flex items-center gap-3 min-h-[44px] px-4 py-3 rounded-lg border border-govbr-gray-border text-left hover:border-govbr-blue" },
                            "aria-pressed": if high_contrast { "true" } else { "false" },
                            onclick: move |_| a11y.toggle_contrast(),
                            Icon { kind: IconKind::Contrast, class: "w-6 h-6 text-govbr-blue shrink-0" }
                            div {
                                p { class: "text-sm font-semibold text-govbr-blue-dark", "Alto contraste" }
                                p { class: "text-xs text-govbr-gray-text", if high_contrast { "Ativado" } else { "Desativado" } }
                            }
                        }

                        button {
                            class: if easy_mode { "flex items-center gap-3 min-h-[44px] px-4 py-3 rounded-lg border-2 border-govbr-blue bg-govbr-blue/5 text-left" } else { "flex items-center gap-3 min-h-[44px] px-4 py-3 rounded-lg border border-govbr-gray-border text-left hover:border-govbr-blue" },
                            "aria-pressed": if easy_mode { "true" } else { "false" },
                            onclick: move |_| a11y.toggle_easy_mode(),
                            Icon { kind: IconKind::BookOpen, class: "w-6 h-6 text-govbr-blue shrink-0" }
                            div {
                                p { class: "text-sm font-semibold text-govbr-blue-dark", "Modo fácil" }
                                p { class: "text-xs text-govbr-gray-text", "Mais espaçamento e botões maiores, foco em passo a passo" }
                            }
                        }
                    }
                }
            }
        }
    }
}
