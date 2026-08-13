use dioxus::prelude::*;

use crate::data::{fraud_level, fraud_risk_score, fraud_signals, FraudRiskLevel};

/// Card de transparência do sistema antifraude: a decisão combina vários sinais
/// independentes (defesa em profundidade), nunca uma regra única isolada.
#[component]
pub fn FraudShield() -> Element {
    let score = fraud_risk_score();
    let level = fraud_level(score);
    let signals = fraud_signals();

    let (level_label, level_color, bar_color) = match level {
        FraudRiskLevel::Normal => ("Normal", "text-govbr-green", "bg-govbr-green"),
        FraudRiskLevel::Monitoring => ("Em monitoramento", "text-amber-700", "bg-amber-500"),
        FraudRiskLevel::Verification => (
            "Verificação adicional necessária",
            "text-orange-700",
            "bg-orange-600",
        ),
        FraudRiskLevel::Blocked => ("Bloqueio e revisão humana", "text-red-700", "bg-red-600"),
    };

    rsx! {
        div { class: "mt-8 bg-govbr-gray-bg rounded-lg p-6",
            div { class: "flex items-center justify-between gap-3 mb-1",
                h2 { class: "text-sm font-semibold text-govbr-blue-dark", "🛡️ Segurança da sua conta" }
                span { class: "text-xs font-bold {level_color}", "{level_label}" }
            }
            div { class: "flex items-center gap-3 mt-3",
                div { class: "flex-1 h-2 rounded-full bg-govbr-gray-border overflow-hidden",
                    div { class: "h-full {bar_color}", style: "width: {score}%" }
                }
                span { class: "text-sm font-bold text-govbr-blue-dark", "{score}/100" }
            }
            p { class: "mt-2 text-xs text-govbr-gray-text",
                "Avaliação combinando vários sinais independentes de comportamento, não uma regra única — assim como sistemas antifraude reais."
            }
            div { class: "mt-4 flex flex-col gap-2.5",
                for s in signals.iter() {
                    div { class: "flex items-start gap-2",
                        span {
                            class: if s.ok { "text-govbr-green font-bold" } else { "text-amber-600 font-bold" },
                            if s.ok { "✓" } else { "⚠" }
                        }
                        div {
                            p { class: "text-xs font-medium text-govbr-blue-dark", "{s.label}" }
                            p { class: "text-xs text-govbr-gray-text", "{s.detail}" }
                        }
                    }
                }
            }
        }
    }
}
