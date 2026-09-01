use dioxus::prelude::*;

use crate::i18n::{t, I18n};

#[component]
pub fn Footer() -> Element {
    let i18n = use_context::<I18n>();
    let locale = (i18n.locale)();

    rsx! {
        footer { id: "ajuda", class: "scroll-mt-16 bg-govbr-blue-dark",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                div { class: "grid sm:grid-cols-2 lg:grid-cols-5 gap-10",
                    div { class: "lg:col-span-1",
                        div { class: "flex items-center gap-2 mb-4",
                            span { class: "text-2xl", "🛡️" }
                            span { class: "text-lg font-bold text-white", "hack" }
                            span { class: "text-lg font-bold text-govbr-blue-soft", ".gov" }
                            span { class: "text-lg font-bold text-govbr-yellow", ".br" }
                        }
                        p { class: "text-sm text-govbr-blue-soft leading-relaxed",
                            {t(locale, "footer.tagline")}
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", {t(locale, "footer.about_title")} }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Órgãos do governo" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Acesso à informação" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Dados abertos" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Eixos temáticos" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Painel de monitoramento" } }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", {t(locale, "footer.programs_title")} }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Notícias" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Participe" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Legislação" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Ouvidoria (Fala.BR)" } }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", {t(locale, "footer.help_title")} }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Central de ajuda" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Acessibilidade" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Mapa do site" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Fale conosco" } }
                            li { span { class: "text-white/50", "Central 0800 000 0000" } }
                        }
                    }

                    div {
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-white mb-4", {t(locale, "footer.legal_title")} }
                        ul { class: "space-y-2 text-sm text-govbr-blue-soft",
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Termos de uso" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Política de privacidade (LGPD)" } }
                            li { a { href: "#", class: "hover:text-govbr-yellow transition-colors", "Política de cookies" } }
                        }
                    }
                }

                div { class: "mt-12 pt-8 border-t border-white/10 flex flex-col sm:flex-row items-center justify-between gap-4",
                    p { class: "text-xs text-govbr-blue-soft", {t(locale, "footer.copyright")} }
                    div { class: "flex items-center gap-4 text-govbr-blue-soft text-sm",
                        a { href: "#", class: "hover:text-govbr-yellow transition-colors", title: "X (Twitter)", "🐦" }
                        a { href: "#", class: "hover:text-govbr-yellow transition-colors", title: "Facebook", "📘" }
                        a { href: "#", class: "hover:text-govbr-yellow transition-colors", title: "Instagram", "📷" }
                        a { href: "#", class: "hover:text-govbr-yellow transition-colors", title: "LinkedIn", "💼" }
                        a { href: "#", class: "hover:text-govbr-yellow transition-colors", title: "YouTube", "▶️" }
                    }
                }
            }
        }
    }
}
