use crate::components::{AlertsBell, Icon, IconKind};
use crate::routes::Route;
use dioxus::prelude::*;

/// Um item do menu principal. Fora da Home, seções como `#servicos` não
/// existem no DOM da página atual — por isso os links de âncora sempre
/// resolvem para `Route::Home` primeiro; a navegação SPA troca a página e
/// o hash é aplicado pelo navegador em seguida. Isso corrige um bug real:
/// antes, clicar em "Segurança" estando em `/minha-vida` não fazia nada,
/// porque a âncora só existe na Home.
struct NavItem {
    label: &'static str,
    href: &'static str,
}

const NAV_ITEMS: [NavItem; 7] = [
    NavItem {
        label: "Serviços urbanos",
        href: "/#servicos-urbanos",
    },
    NavItem {
        label: "Assistente",
        href: "/#assistente",
    },
    NavItem {
        label: "Serviços",
        href: "/#servicos",
    },
    NavItem {
        label: "Categorias",
        href: "/#categorias",
    },
    NavItem {
        label: "Segurança",
        href: "/#seguranca",
    },
    NavItem {
        label: "Status",
        href: "/#status",
    },
    NavItem {
        label: "Ajuda",
        href: "/#ajuda",
    },
];

/// Header institucional, presente em todas as páginas (dentro do `AppLayout`).
/// Contém logo, menu principal, sino de alertas e o botão de login simulado.
#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { class: "sticky top-0 z-50",
            // Barra de pulo de conteúdo (links visíveis só ao navegar por teclado/leitor de tela)
            div { class: "bg-govbr-blue-dark text-xs",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-0 overflow-visible flex items-center gap-4",
                    a {
                        href: "#top",
                        class: "sr-only focus:not-sr-only focus:absolute focus:z-50 focus:bg-white focus:text-govbr-blue-dark focus:outline focus:outline-govbr-yellow focus:px-3 focus:py-2",
                        "Ir para o conteúdo",
                    }
                    a {
                        href: "#menu-principal",
                        class: "sr-only focus:not-sr-only focus:absolute focus:z-50 focus:bg-white focus:text-govbr-blue-dark focus:outline focus:outline-govbr-yellow focus:px-3 focus:py-2",
                        "Ir para o menu",
                    }
                    a {
                        href: "#ajuda",
                        class: "sr-only focus:not-sr-only focus:absolute focus:z-50 focus:bg-white focus:text-govbr-blue-dark focus:outline focus:outline-govbr-yellow focus:px-3 focus:py-2",
                        "Ir para rodapé",
                    }
                }
            }

            // Header principal
            header { class: "border-b border-govbr-gray-border bg-white",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex items-center justify-between h-16 sm:h-20 gap-2 sm:gap-4",
                        // Logo — antes era `a { href: "#top" }`, que em qualquer
                        // rota fora da Home só mudava a hash da URL sem navegar
                        // (o bug que você reportou: clicar na logo em
                        // /servicos/cnh-digital#top não levava pra Home). Agora
                        // é navegação de verdade via router.
                        Link { to: Route::Home {}, class: "flex items-center gap-2 shrink-0",
                            Icon { kind: IconKind::Shield, class: "w-6 h-6 sm:w-7 sm:h-7 text-govbr-blue" }
                            span { class: "text-lg sm:text-xl font-bold text-govbr-gray-text tracking-tight", "hack" }
                            span { class: "text-lg sm:text-xl font-bold text-govbr-blue tracking-tight", ".gov" }
                            span { class: "text-lg sm:text-xl font-bold text-govbr-yellow tracking-tight", ".br" }
                        }

                        // Menu desktop
                        nav { id: "menu-principal", class: "hidden md:flex items-center gap-8",
                            for item in NAV_ITEMS {
                                a {
                                    href: "{item.href}",
                                    class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors",
                                    "{item.label}"
                                }
                            }
                            Link { to: Route::MyGovPanel {}, class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", "Minha vida" }
                        }

                        div { class: "flex items-center gap-1.5 sm:gap-3",
                            AlertsBell {}
                            a {
                                href: "/#servicos",
                                class: "hidden lg:flex items-center gap-2 text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                Icon { kind: IconKind::Search, class: "w-4 h-4" }
                                "Atalhos"
                            }
                            // Botão de login: em telas pequenas, mostra só o ícone para não
                            // disputar espaço com o sino de alertas e o menu hambúrguer
                            // (bug de responsividade corrigido — antes o texto completo
                            // "Entrar com gov.br" nunca era escondido no mobile).
                            button {
                                class: "flex items-center gap-2 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-3 sm:px-4 py-2",
                                "aria-label": "Entrar com gov.br",
                                Icon { kind: IconKind::User, class: "w-4 h-4" }
                                span { class: "hidden sm:inline", "Entrar com gov.br" }
                            }
                            button {
                                class: "md:hidden flex items-center justify-center w-9 h-9 text-govbr-blue-dark",
                                "aria-label": if menu_open() { "Fechar menu" } else { "Abrir menu" },
                                "aria-expanded": if menu_open() { "true" } else { "false" },
                                onclick: move |_| menu_open.toggle(),
                                if menu_open() {
                                    Icon { kind: IconKind::Close, class: "w-6 h-6" }
                                } else {
                                    Icon { kind: IconKind::Menu, class: "w-6 h-6" }
                                }
                            }
                        }
                    }

                    // Menu mobile
                    if menu_open() {
                        nav { class: "md:hidden flex flex-col gap-1 pb-4",
                            for item in NAV_ITEMS {
                                a {
                                    href: "{item.href}",
                                    class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2.5 min-h-[44px] flex items-center",
                                    onclick: move |_| menu_open.set(false),
                                    "{item.label}"
                                }
                            }
                            Link {
                                to: Route::MyGovPanel {},
                                class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2.5 min-h-[44px] flex items-center",
                                onclick: move |_| menu_open.set(false),
                                "Minha vida"
                            }
                        }
                    }
                }
            }
        }
    }
}
