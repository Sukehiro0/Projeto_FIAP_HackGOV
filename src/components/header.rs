use crate::accessibility::A11ySettings;
use crate::components::AlertsBell;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);
    let mut a11y = use_context::<A11ySettings>();
    let high_contrast = (a11y.high_contrast)();
    let easy_mode = (a11y.easy_mode)();

    rsx! {
        div { class: "sticky top-0 z-50",
            // Barra superior institucional (links de pulo + acessibilidade) - estilo gov.br real: branca, discreta
            div { class: "bg-white border-b border-govbr-gray-border text-xs",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-9 flex items-center justify-between gap-4",
                    div { class: "flex items-center gap-4",
                        a {
                            href: "#top",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            "Ir para o conteúdo",
                        }
                        a {
                            href: "#menu-principal",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            "Ir para o menu",
                        }
                        a {
                            href: "#ajuda",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            "Ir para rodapé",
                        }
                    }
                    div { class: "flex items-center gap-3 ml-auto text-govbr-blue-dark",
                        span { class: "hidden sm:inline text-govbr-gray-text", "Acessibilidade:" }
                        button {
                            class: "hover:underline font-semibold",
                            title: "Diminuir fonte",
                            onclick: move |_| a11y.decrease_font(),
                            "A-"
                        }
                        button {
                            class: "hover:underline font-semibold",
                            title: "Tamanho padrão",
                            onclick: move |_| a11y.reset_font(),
                            "A"
                        }
                        button {
                            class: "hover:underline font-semibold",
                            title: "Aumentar fonte",
                            onclick: move |_| a11y.increase_font(),
                            "A+"
                        }
                        span { class: "hidden sm:inline text-govbr-gray-border", "|" }
                        button {
                            class: "hidden sm:inline hover:underline",
                            "aria-pressed": if high_contrast { "true" } else { "false" },
                            onclick: move |_| a11y.toggle_contrast(),
                            if high_contrast { "Desativar alto contraste" } else { "◐ Alto contraste" }
                        }
                        span { class: "hidden sm:inline text-govbr-gray-border", "|" }
                        button {
                            class: "hidden sm:inline hover:underline",
                            "aria-pressed": if easy_mode { "true" } else { "false" },
                            onclick: move |_| a11y.toggle_easy_mode(),
                            if easy_mode { "Desativar modo fácil" } else { "🐢 Modo fácil" }
                        }
                    }
                }
            }

            // Header principal
            header { class: "border-b border-govbr-gray-border bg-white",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex items-center justify-between h-20 gap-4",
                        // Logo
                        a { href: "#top", class: "flex items-center gap-2 shrink-0",
                            span { class: "text-2xl", "🛡️" }
                            span { class: "text-xl font-bold text-govbr-gray-text tracking-tight", "hack" }
                            span { class: "text-xl font-bold text-govbr-blue tracking-tight", ".gov" }
                            span { class: "text-xl font-bold text-govbr-yellow tracking-tight", ".br" }
                        }

                        // Menu desktop
                        nav { id: "menu-principal", class: "hidden md:flex items-center gap-8",
                            a { href: "#assistente", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Assistente" }
                            Link { to: Route::MyGovPanel {}, class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Minha vida" }
                            a { href: "#servicos", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Serviços" }
                            a { href: "#categorias", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Categorias" }
                            a { href: "#seguranca", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Segurança" }
                            a { href: "#status", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Status" }
                            a { href: "#ajuda", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Ajuda" }
                        }

                        div { class: "flex items-center gap-3",
                            AlertsBell {}
                            a {
                                href: "#servicos",
                                class: "hidden sm:flex items-center gap-2 text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                "▦ Atalhos"
                            }
                            button { class: "flex items-center gap-2 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-4 py-2",
                                span { class: "flex items-center justify-center w-5 h-5 rounded-full bg-white/20 text-xs", "👤" }
                                "Entrar com gov.br"
                            }
                            button {
                                class: "md:hidden text-govbr-blue-dark text-xl",
                                onclick: move |_| menu_open.toggle(),
                                "☰"
                            }
                        }
                    }

                    // Menu mobile
                    if menu_open() {
                        nav { class: "md:hidden flex flex-col gap-1 pb-4",
                            a { href: "#assistente", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Assistente" }
                            Link { to: Route::MyGovPanel {}, class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Minha vida" }
                            a { href: "#servicos", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Serviços" }
                            a { href: "#categorias", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Categorias" }
                            a { href: "#seguranca", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Segurança" }
                            a { href: "#status", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Status" }
                            a { href: "#ajuda", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", "Ajuda" }
                        }
                    }
                }
            }
        }
    }
}
