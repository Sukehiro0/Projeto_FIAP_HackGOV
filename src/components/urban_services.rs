use dioxus::prelude::*;

use crate::components::{Icon, IconKind};
use crate::data::urban_services;
use crate::routes::Route;

/// Banner de destaque do módulo "Cidade e Serviços Urbanos" — colocado logo
/// depois do Hero, propositalmente antes de qualquer outra seção, porque foi
/// pedido que esse módulo novo ficasse fácil de achar e em destaque. Visual
/// diferenciado (fundo azul escuro, selo "Novo") para não se misturar com o
/// resto das seções brancas/cinza da página.
#[component]
pub fn UrbanServices() -> Element {
    let services = urban_services();

    rsx! {
        section { id: "servicos-urbanos", class: "reveal scroll-mt-16 bg-govbr-blue-dark",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14",
                div { class: "flex flex-wrap items-center gap-3 mb-2",
                    span { class: "inline-flex items-center gap-1.5 text-xs font-bold text-govbr-blue-dark bg-govbr-yellow rounded-full px-3 py-1",
                        Icon { kind: IconKind::Sparkle, class: "w-3.5 h-3.5" }
                        "Novo módulo"
                    }
                }
                h2 { class: "text-2xl sm:text-3xl font-bold text-white mb-2", "Cidade e Serviços Urbanos" }
                p { class: "text-govbr-blue-soft max-w-2xl mb-8",
                    "Viu um problema na sua rua? Relate direto aqui — iluminação apagada, buraco na via, árvore com risco de queda ou entulho jogado — e acompanhe o andamento pelo protocolo, do mesmo jeito que os outros serviços do HackGov."
                }

                div { class: "grid grid-cols-2 lg:grid-cols-4 gap-4",
                    for s in services.iter() {
                        Link {
                            to: Route::ServiceDetail { slug: s.slug.to_string() },
                            class: "flex flex-col gap-3 bg-white/10 hover:bg-white/20 transition-colors rounded-lg p-5",
                            Icon { kind: s.icon, class: "w-7 h-7 text-govbr-yellow" }
                            div {
                                p { class: "text-sm font-semibold text-white", "{s.name}" }
                                p { class: "text-xs text-govbr-blue-soft mt-1", "Tempo estimado: {s.time_estimate}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
