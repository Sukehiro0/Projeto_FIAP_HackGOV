use crate::accessibility::A11ySettings;
use crate::auth::AuthState;
use crate::components::{AlertsBell, AuthModal};
use crate::i18n::{t, I18n};
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);
    let mut a11y = use_context::<A11ySettings>();
    let high_contrast = (a11y.high_contrast)();
    let mut auth = use_context::<AuthState>();
    let mut i18n = use_context::<I18n>();
    let locale = (i18n.locale)();
    let mut show_auth_modal = use_signal(|| false);
    let current_user = (auth.user)();

    rsx! {
        div { class: "sticky top-0 z-50",
            // Barra superior institucional (links de pulo + acessibilidade) - estilo gov.br real: branca, discreta
            div { class: "bg-white border-b border-govbr-gray-border text-xs",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-9 flex items-center justify-between gap-4",
                    div { class: "flex items-center gap-4",
                        a {
                            href: "#top",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            {t(locale, "skip.content")}
                        }
                        a {
                            href: "#menu-principal",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            {t(locale, "skip.menu")}
                        }
                        a {
                            href: "#ajuda",
                            class: "sr-only focus:not-sr-only focus:outline focus:outline-govbr-blue focus:px-2",
                            {t(locale, "skip.footer")}
                        }
                    }
                    div { class: "flex items-center gap-3 ml-auto text-govbr-blue-dark",
                        span { class: "hidden sm:inline text-govbr-gray-text", {t(locale, "a11y.label")} }
                        button {
                            class: "hover:underline font-semibold",
                            title: t(locale, "a11y.decrease_title"),
                            onclick: move |_| a11y.decrease_font(),
                            "A-"
                        }
                        button {
                            class: "hover:underline font-semibold",
                            title: t(locale, "a11y.reset_title"),
                            onclick: move |_| a11y.reset_font(),
                            "A"
                        }
                        button {
                            class: "hover:underline font-semibold",
                            title: t(locale, "a11y.increase_title"),
                            onclick: move |_| a11y.increase_font(),
                            "A+"
                        }
                        span { class: "hidden sm:inline text-govbr-gray-border", "|" }
                        button {
                            class: "hidden sm:inline hover:underline",
                            "aria-pressed": if high_contrast { "true" } else { "false" },
                            onclick: move |_| a11y.toggle_contrast(),
                            if high_contrast { {t(locale, "a11y.contrast_on")} } else { {t(locale, "a11y.contrast_off")} }
                        }
                        span { class: "hidden sm:inline text-govbr-gray-border", "|" }
                        button {
                            class: "hover:underline font-semibold",
                            title: t(locale, "lang.switch_title"),
                            onclick: move |_| i18n.toggle(),
                            "🌐 {locale.label()}"
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
                            a { href: "#assistente", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.assistant")} }
                            Link { to: Route::MyGovPanel {}, class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.my_life")} }
                            a { href: "#servicos", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.services")} }
                            a { href: "#categorias", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.categories")} }
                            a { href: "#seguranca", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.security")} }
                            a { href: "#status", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.status")} }
                            a { href: "#ajuda", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue transition-colors", {t(locale, "nav.help")} }
                        }

                        div { class: "flex items-center gap-3",
                            AlertsBell {}
                            a {
                                href: "#servicos",
                                class: "hidden sm:flex items-center gap-2 text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                {t(locale, "nav.shortcuts")}
                            }
                            if let Some(user) = current_user {
                                span { class: "hidden sm:inline text-sm text-govbr-gray-text",
                                    {t(locale, "auth.hello")} " "
                                    span { class: "font-semibold text-govbr-blue-dark", "{user.username}" }
                                }
                                button {
                                    class: "flex items-center gap-2 text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                    onclick: move |_| {
                                        spawn(async move {
                                            auth.logout().await;
                                        });
                                    },
                                    {t(locale, "auth.logout")}
                                }
                            } else {
                                button {
                                    class: "flex items-center gap-2 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-4 py-2",
                                    onclick: move |_| show_auth_modal.set(true),
                                    span { class: "flex items-center justify-center w-5 h-5 rounded-full bg-white/20 text-xs", "👤" }
                                    {t(locale, "auth.login_cta")}
                                }
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
                            a { href: "#assistente", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.assistant")} }
                            Link { to: Route::MyGovPanel {}, class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.my_life")} }
                            a { href: "#servicos", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.services")} }
                            a { href: "#categorias", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.categories")} }
                            a { href: "#seguranca", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.security")} }
                            a { href: "#status", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.status")} }
                            a { href: "#ajuda", class: "text-sm font-medium text-govbr-gray-text hover:text-govbr-blue py-2", {t(locale, "nav.help")} }
                        }
                    }
                }
            }
        }

        AuthModal { show: show_auth_modal }
    }
}
