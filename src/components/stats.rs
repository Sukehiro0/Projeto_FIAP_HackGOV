use dioxus::prelude::*;

struct Stat {
    value: &'static str,
    label: &'static str,
}

#[component]
pub fn Stats() -> Element {
    let stats = [
        Stat {
            value: "5.6 mil+",
            label: "Serviços no portal",
        },
        Stat {
            value: "93%",
            label: "Serviços 100% digitais",
        },
        Stat {
            value: "128M+",
            label: "Contas verificadas",
        },
        Stat {
            value: "99.9%",
            label: "Disponibilidade da plataforma",
        },
    ];

    rsx! {
        section { class: "reveal border-b border-govbr-gray-border bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                div { class: "grid grid-cols-2 md:grid-cols-4 gap-8",
                    for stat in stats {
                        div { class: "text-center",
                            p { class: "text-3xl sm:text-4xl font-extrabold text-govbr-blue", "{stat.value}" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "{stat.label}" }
                        }
                    }
                }
            }
        }
    }
}
