use dioxus::prelude::*;

use crate::components::Icon;
use crate::data::categories;
use crate::ui_state::UiPanels;

/// Grade de categorias de serviço, para navegação por assunto em vez de por órgão.
///
/// Só a categoria "Cidade e Serviços Urbanos" tem conteúdo real por trás (é o
/// módulo novo) — por isso só ela navega de verdade. As demais ainda não têm
/// nenhuma página associada, então em vez de fingir com `href="#"` (que deixa
/// a URL com uma "#" feia ao passar o mouse), o clique abre um aviso honesto
/// de simulação.
#[component]
pub fn Categories() -> Element {
    let mut ui = use_context::<UiPanels>();

    rsx! {
        section { id: "categorias", class: "reveal scroll-mt-16 bg-white border-y border-govbr-gray-border",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                h2 { class: "text-2xl sm:text-3xl font-bold text-govbr-blue-dark mb-2 text-center", "Navegue por categoria" }
                p { class: "text-govbr-gray-text mb-10 text-center", "Encontre o serviço ou a informação que você precisa" }

                div { class: "grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4",
                    for category in categories() {
                        if category.name == "Cidade e Serviços Urbanos" {
                            a {
                                href: "/#servicos-urbanos",
                                class: "relative flex flex-col items-center justify-center gap-3 text-center bg-govbr-blue/5 border-2 border-govbr-blue hover:bg-govbr-blue/10 rounded p-6 sm:p-8 transition-colors",
                                span { class: "absolute top-2 right-2 text-[10px] font-bold text-white bg-govbr-blue rounded-full px-2 py-0.5", "Novo" }
                                Icon { kind: category.icon, class: "w-8 h-8 text-govbr-blue" }
                                span { class: "text-xs sm:text-sm font-bold uppercase tracking-wide text-govbr-blue-dark", "{category.name}" }
                            }
                        } else {
                            button {
                                class: "flex flex-col items-center justify-center gap-3 text-center bg-govbr-gray-bg hover:bg-govbr-gray-border/50 rounded p-6 sm:p-8 transition-colors",
                                onclick: move |_| ui.coming_soon.set(Some(category.name)),
                                Icon { kind: category.icon, class: "w-8 h-8 text-govbr-blue" }
                                span { class: "text-xs sm:text-sm font-bold uppercase tracking-wide text-govbr-blue-dark", "{category.name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
