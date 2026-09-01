//! Internacionalização (i18n) simples: um enum `Locale`, um contexto
//! reativo (`I18n`) e uma função `t(locale, key)` que devolve o texto no
//! idioma atual. Português (Brasil) é o idioma padrão e principal do
//! projeto; Inglês (EUA) foi adicionado como segundo idioma.
//!
//! Cobertura atual: cabeçalho, rodapé (títulos), seção inicial (hero) e o
//! modal de login/cadastro. O restante do conteúdo (catálogo de serviços,
//! páginas internas) permanece em pt-BR — a infraestrutura aqui já permite
//! estender a cobertura gradualmente, adicionando novas entradas ao `match`
//! abaixo.

use dioxus::prelude::*;

/// Idiomas suportados pela interface.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Locale {
    #[default]
    PtBr,
    EnUs,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::PtBr => "PT",
            Locale::EnUs => "EN",
        }
    }

    pub fn other(&self) -> Locale {
        match self {
            Locale::PtBr => Locale::EnUs,
            Locale::EnUs => Locale::PtBr,
        }
    }

    fn storage_code(&self) -> &'static str {
        match self {
            Locale::PtBr => "pt-BR",
            Locale::EnUs => "en-US",
        }
    }
}

const LOCALE_STORAGE_KEY: &str = "hackgov_locale";

/// Contexto de idioma compartilhado por toda a aplicação.
#[derive(Clone, Copy)]
pub struct I18n {
    pub locale: Signal<Locale>,
}

impl I18n {
    pub fn provide() -> Self {
        let state = Self {
            locale: Signal::new(Locale::default()),
        };
        let provided: Self = use_context_provider(|| state);

        let mut restore_handle = provided;
        use_effect(move || {
            spawn(async move {
                if let Some(code) = read_locale_from_storage().await {
                    if code == Locale::EnUs.storage_code() {
                        restore_handle.locale.set(Locale::EnUs);
                    }
                }
            });
        });

        provided
    }

    /// Alterna entre os dois idiomas suportados e persiste a escolha.
    pub fn toggle(&mut self) {
        let next = self.locale.read().other();
        self.locale.set(next);
        spawn(async move {
            save_locale_to_storage(next.storage_code()).await;
        });
    }
}

async fn read_locale_from_storage() -> Option<String> {
    let mut eval = document::eval(&format!(
        "dioxus.send(window.localStorage.getItem('{LOCALE_STORAGE_KEY}'));"
    ));
    let value: Result<Option<String>, _> = eval.recv().await;
    value.unwrap_or(None)
}

async fn save_locale_to_storage(code: &str) {
    let _ = document::eval(&format!(
        "window.localStorage.setItem('{LOCALE_STORAGE_KEY}', '{code}');"
    ))
    .await;
}

/// Busca a tradução de `key` no idioma `locale`. Quando não há entrada
/// cadastrada, devolve a própria chave — assim nenhuma string some
/// silenciosamente, e chaves esquecidas ficam visíveis durante o
/// desenvolvimento.
pub fn t(locale: Locale, key: &str) -> &str {
    use Locale::*;
    match (locale, key) {
        // ---- Barra de acessibilidade ----
        (PtBr, "skip.content") => "Ir para o conteúdo",
        (EnUs, "skip.content") => "Skip to content",
        (PtBr, "skip.menu") => "Ir para o menu",
        (EnUs, "skip.menu") => "Skip to menu",
        (PtBr, "skip.footer") => "Ir para rodapé",
        (EnUs, "skip.footer") => "Skip to footer",
        (PtBr, "a11y.label") => "Acessibilidade:",
        (EnUs, "a11y.label") => "Accessibility:",
        (PtBr, "a11y.decrease_title") => "Diminuir fonte",
        (EnUs, "a11y.decrease_title") => "Decrease font size",
        (PtBr, "a11y.reset_title") => "Tamanho padrão",
        (EnUs, "a11y.reset_title") => "Default size",
        (PtBr, "a11y.increase_title") => "Aumentar fonte",
        (EnUs, "a11y.increase_title") => "Increase font size",
        (PtBr, "a11y.contrast_on") => "Desativar alto contraste",
        (EnUs, "a11y.contrast_on") => "Disable high contrast",
        (PtBr, "a11y.contrast_off") => "◐ Alto contraste",
        (EnUs, "a11y.contrast_off") => "◐ High contrast",
        (PtBr, "lang.switch_title") => "Alterar idioma",
        (EnUs, "lang.switch_title") => "Switch language",

        // ---- Navegação principal ----
        (PtBr, "nav.assistant") => "Assistente",
        (EnUs, "nav.assistant") => "Assistant",
        (PtBr, "nav.my_life") => "Minha vida",
        (EnUs, "nav.my_life") => "My life",
        (PtBr, "nav.services") => "Serviços",
        (EnUs, "nav.services") => "Services",
        (PtBr, "nav.categories") => "Categorias",
        (EnUs, "nav.categories") => "Categories",
        (PtBr, "nav.security") => "Segurança",
        (EnUs, "nav.security") => "Security",
        (PtBr, "nav.status") => "Status",
        (EnUs, "nav.status") => "Status",
        (PtBr, "nav.help") => "Ajuda",
        (EnUs, "nav.help") => "Help",
        (PtBr, "nav.shortcuts") => "▦ Atalhos",
        (EnUs, "nav.shortcuts") => "▦ Shortcuts",

        // ---- Autenticação ----
        (PtBr, "auth.login_cta") => "Entrar com gov.br",
        (EnUs, "auth.login_cta") => "Sign in with gov.br",
        (PtBr, "auth.logout") => "Sair",
        (EnUs, "auth.logout") => "Log out",
        (PtBr, "auth.hello") => "Olá,",
        (EnUs, "auth.hello") => "Hi,",
        (PtBr, "auth.modal_title_login") => "Entrar com gov.br",
        (EnUs, "auth.modal_title_login") => "Sign in with gov.br",
        (PtBr, "auth.modal_title_signup") => "Criar conta gov.br",
        (EnUs, "auth.modal_title_signup") => "Create a gov.br account",
        (PtBr, "auth.username") => "Usuário",
        (EnUs, "auth.username") => "Username",
        (PtBr, "auth.password") => "Senha",
        (EnUs, "auth.password") => "Password",
        (PtBr, "auth.submit_login") => "Entrar",
        (EnUs, "auth.submit_login") => "Sign in",
        (PtBr, "auth.submit_signup") => "Criar conta",
        (EnUs, "auth.submit_signup") => "Create account",
        (PtBr, "auth.switch_to_signup") => "Ainda não tem conta? Cadastre-se",
        (EnUs, "auth.switch_to_signup") => "Don't have an account? Sign up",
        (PtBr, "auth.switch_to_login") => "Já tem conta? Entrar",
        (EnUs, "auth.switch_to_login") => "Already have an account? Sign in",
        (PtBr, "auth.close") => "Fechar",
        (EnUs, "auth.close") => "Close",
        (PtBr, "auth.loading") => "Enviando...",
        (EnUs, "auth.loading") => "Submitting...",
        (PtBr, "auth.disclaimer") => {
            "Conta de demonstração local — não é uma conta real gov.br. Não use senhas reais."
        }
        (EnUs, "auth.disclaimer") => {
            "Local demo account — not a real gov.br account. Do not use a real password."
        }

        // ---- Rodapé ----
        (PtBr, "footer.tagline") => {
            "Portal digital de serviços, mais tecnológico, seguro e simples para o cidadão."
        }
        (EnUs, "footer.tagline") => {
            "A more technological, secure and simple digital services portal for citizens."
        }
        (PtBr, "footer.about_title") => "Sobre o Governo",
        (EnUs, "footer.about_title") => "About the Government",
        (PtBr, "footer.programs_title") => "Ações e Programas",
        (EnUs, "footer.programs_title") => "Actions and Programs",
        (PtBr, "footer.help_title") => "Ajuda",
        (EnUs, "footer.help_title") => "Help",
        (PtBr, "footer.legal_title") => "Legal",
        (EnUs, "footer.legal_title") => "Legal",
        (PtBr, "footer.copyright") => {
            "© 2026 HackGOV. Projeto de demonstração — não é um site oficial do governo."
        }
        (EnUs, "footer.copyright") => {
            "© 2026 HackGOV. Demo project — not an official government website."
        }

        // ---- Hero / busca ----
        (PtBr, "hero.title") => "Serviços e Informações do Brasil",
        (EnUs, "hero.title") => "Services and Information from Brazil",
        (PtBr, "hero.search_placeholder") => "O que você procura?",
        (EnUs, "hero.search_placeholder") => "What are you looking for?",
        (PtBr, "hero.search_aria") => "Buscar",
        (EnUs, "hero.search_aria") => "Search",
        (PtBr, "hero.no_results") => "Nenhum serviço encontrado para",
        (EnUs, "hero.no_results") => "No service found for",

        _ => key,
    }
}
