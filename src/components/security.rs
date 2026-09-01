use dioxus::prelude::*;

use crate::components::{Icon, IconKind};

struct Feature {
    icon: IconKind,
    title: &'static str,
    description: &'static str,
}

/// Seção que lista, de forma transparente, as medidas de segurança da plataforma.
#[component]
pub fn Security() -> Element {
    let features = [
        Feature {
            icon: IconKind::Lock,
            title: "Criptografia de ponta a ponta",
            description: "Todos os seus dados trafegam criptografados, do seu dispositivo até os servidores oficiais.",
        },
        Feature {
            icon: IconKind::Fingerprint,
            title: "Autenticação em duas etapas",
            description: "Proteção extra na sua conta com verificação biométrica ou por aplicativo autenticador.",
        },
        Feature {
            icon: IconKind::Document,
            title: "Conformidade com a LGPD",
            description: "Seus dados pessoais são tratados com transparência e nunca compartilhados sem consentimento.",
        },
        Feature {
            icon: IconKind::Activity,
            title: "Monitoramento 24/7",
            description: "Uma equipe de segurança acompanha a plataforma o tempo todo contra ameaças e falhas.",
        },
    ];

    rsx! {
        section { id: "seguranca", class: "reveal scroll-mt-16 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            div { class: "text-center max-w-2xl mx-auto mb-12",
                span { class: "inline-flex items-center gap-2 text-xs font-semibold text-govbr-green bg-govbr-green/10 border border-govbr-green/20 rounded px-3 py-1 mb-4",
                    Icon { kind: IconKind::Shield, class: "w-3.5 h-3.5" }
                    "Segurança em primeiro lugar"
                }
                h2 { class: "text-2xl sm:text-3xl font-bold text-govbr-blue-dark", "Sua segurança é nossa prioridade" }
                p { class: "mt-3 text-govbr-gray-text", "Tecnologia de ponta para proteger seus dados e sua identidade digital." }
            }

            div { class: "grid sm:grid-cols-2 lg:grid-cols-4 gap-4",
                for feature in features {
                    div { class: "flex flex-col items-center text-center gap-3 bg-govbr-gray-bg rounded p-6",
                        Icon { kind: feature.icon, class: "w-8 h-8 text-govbr-blue" }
                        h3 { class: "text-base font-semibold text-govbr-blue-dark", "{feature.title}" }
                        p { class: "text-sm text-govbr-gray-text leading-relaxed", "{feature.description}" }
                    }
                }
            }
        }
    }
}
