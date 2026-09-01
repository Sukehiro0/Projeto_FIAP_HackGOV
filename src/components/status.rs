use dioxus::prelude::*;

use crate::components::{Icon, IconKind};

struct Indicator {
    icon: IconKind,
    label: &'static str,
    value: &'static str,
}

/// Painel de status/transparência: disponibilidade, incidentes e auditoria em números.
#[component]
pub fn StatusPanel() -> Element {
    let indicators = [
        Indicator {
            icon: IconKind::CheckCircle,
            label: "Disponibilidade da plataforma",
            value: "99.98% nos últimos 30 dias",
        },
        Indicator {
            icon: IconKind::Activity,
            label: "Incidentes de segurança",
            value: "0 nos últimos 90 dias",
        },
        Indicator {
            icon: IconKind::Lock,
            label: "Certificação",
            value: "TLS 1.3 + criptografia AES-256",
        },
        Indicator {
            icon: IconKind::Document,
            label: "Auditoria externa",
            value: "Última verificação: julho/2026",
        },
    ];

    rsx! {
        section { id: "status", class: "reveal scroll-mt-16 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            div { class: "text-center max-w-2xl mx-auto mb-12",
                span { class: "inline-flex items-center gap-2 text-xs font-semibold text-govbr-green bg-govbr-green/10 border border-govbr-green/20 rounded px-3 py-1 mb-4",
                    Icon { kind: IconKind::Activity, class: "w-3.5 h-3.5" }
                    "Transparência em tempo real"
                }
                h2 { class: "text-2xl sm:text-3xl font-bold text-govbr-blue-dark", "Status e transparência da plataforma" }
                p { class: "mt-3 text-govbr-gray-text",
                    "Diferente de outros portais, mostramos abertamente como estamos protegendo seus dados."
                }
            }

            div { class: "grid sm:grid-cols-2 lg:grid-cols-4 gap-4",
                for ind in indicators {
                    div { class: "bg-govbr-gray-bg rounded p-6 text-center",
                        Icon { kind: ind.icon, class: "w-8 h-8 text-govbr-green mx-auto" }
                        p { class: "mt-3 text-sm font-medium text-govbr-gray-text", "{ind.label}" }
                        p { class: "mt-1 text-base font-bold text-govbr-blue-dark", "{ind.value}" }
                    }
                }
            }
        }
    }
}
