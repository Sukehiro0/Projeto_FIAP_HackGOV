use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::ui_state::UiPanels;

/// Aviso exibido ao clicar em um link institucional que ainda não tem página
/// real (ex.: "Termos de uso", "Dados abertos"). Em vez de um link morto com
/// `#` na URL, isso deixa explícito que é uma simulação de hackathon —
/// honesto com o usuário e sem quebrar a experiência de navegação.
#[component]
pub fn ComingSoonDialog() -> Element {
    let mut ui = use_context::<UiPanels>();
    let label = (ui.coming_soon)();

    if let Some(label) = label {
        rsx! {
            div {
                class: "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40 p-4",
                onclick: move |_| ui.coming_soon.set(None),
                div {
                    class: "bg-white rounded-lg shadow-2xl w-full max-w-sm p-6 text-center",
                    onclick: move |e| e.stop_propagation(),
                    role: "dialog",
                    "aria-label": "Funcionalidade em simulação",
                    Icon { kind: IconKind::Tool, class: "w-9 h-9 text-govbr-blue mx-auto" }
                    h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "\"{label}\" — em simulação" }
                    p { class: "mt-2 text-sm text-govbr-gray-text leading-relaxed",
                        "Este é um protótipo de hackathon (HackGov). Em um sistema real, aqui entraria a página completa de \"{label}\". Por enquanto essa parte é só demonstrativa."
                    }
                    button {
                        class: "mt-5 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-6 py-2.5",
                        onclick: move |_| ui.coming_soon.set(None),
                        "Entendi"
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
