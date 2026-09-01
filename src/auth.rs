//! Estado de autenticação compartilhado por toda a aplicação (cliente).
//!
//! O token de sessão retornado pelo backend (ver `src/api.rs` e
//! `src/server.rs`) é guardado no `localStorage` do navegador para
//! sobreviver a recarregamentos de página. Isso é feito via `document::eval`
//! dentro de um efeito (só roda após a hidratação, nunca durante a
//! renderização no servidor).

use dioxus::prelude::*;

use crate::api::{self, PublicUser};

const TOKEN_STORAGE_KEY: &str = "hackgov_session_token";

/// Estado de autenticação compartilhado por toda a aplicação.
#[derive(Clone, Copy)]
pub struct AuthState {
    pub user: Signal<Option<PublicUser>>,
    token: Signal<Option<String>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl AuthState {
    pub fn provide() -> Self {
        let state = Self {
            user: Signal::new(None),
            token: Signal::new(None),
            loading: Signal::new(true),
            error: Signal::new(None),
        };
        let provided: Self = use_context_provider(|| state);

        // Restaura a sessão salva no localStorage, uma única vez, após a
        // hidratação (código de navegador não pode rodar durante a SSR).
        let mut restore_handle = provided;
        use_effect(move || {
            spawn(async move {
                match read_token_from_storage().await {
                    Some(token) => restore_handle.restore_session(token).await,
                    None => restore_handle.loading.set(false),
                }
            });
        });

        provided
    }

    async fn restore_session(&mut self, token: String) {
        match api::me(token.clone()).await {
            Ok(Some(user)) => {
                self.token.set(Some(token));
                self.user.set(Some(user));
            }
            _ => {
                clear_token_storage().await;
            }
        }
        self.loading.set(false);
    }

    /// Efetua login. Em caso de erro, também grava a mensagem em `self.error`.
    pub async fn login(&mut self, username: String, password: String) -> Result<(), String> {
        self.error.set(None);
        match api::login(username, password).await {
            Ok(resp) => {
                save_token_to_storage(&resp.token).await;
                self.token.set(Some(resp.token));
                self.user.set(Some(resp.user));
                Ok(())
            }
            Err(e) => {
                let msg = friendly_error(&e.to_string());
                self.error.set(Some(msg.clone()));
                Err(msg)
            }
        }
    }

    /// Cria a conta e já efetua login. Em caso de erro, também grava a
    /// mensagem em `self.error`.
    pub async fn signup(&mut self, username: String, password: String) -> Result<(), String> {
        self.error.set(None);
        match api::signup(username, password).await {
            Ok(resp) => {
                save_token_to_storage(&resp.token).await;
                self.token.set(Some(resp.token));
                self.user.set(Some(resp.user));
                Ok(())
            }
            Err(e) => {
                let msg = friendly_error(&e.to_string());
                self.error.set(Some(msg.clone()));
                Err(msg)
            }
        }
    }

    pub async fn logout(&mut self) {
        let token = self.token.read().clone();
        if let Some(token) = token {
            let _ = api::logout(token).await;
        }
        clear_token_storage().await;
        self.token.set(None);
        self.user.set(None);
    }
}

/// As mensagens de erro de uma server function chegam ao cliente encapsuladas
/// (ex.: `"error running server function: <msg> (details: None)"`); aqui
/// extraímos só a mensagem original e garantimos que o texto nunca fique vazio.
fn friendly_error(raw: &str) -> String {
    let mut msg = raw.trim();
    if let Some(rest) = msg.strip_prefix("error running server function:") {
        msg = rest.trim();
    }
    if let Some(idx) = msg.find(" (details:") {
        msg = msg[..idx].trim();
    }
    if msg.is_empty() {
        "Não foi possível concluir a operação. Tente novamente.".to_string()
    } else {
        msg.to_string()
    }
}

async fn read_token_from_storage() -> Option<String> {
    let mut eval = document::eval(&format!(
        "dioxus.send(window.localStorage.getItem('{TOKEN_STORAGE_KEY}'));"
    ));
    let value: Result<Option<String>, _> = eval.recv().await;
    value.unwrap_or(None)
}

async fn save_token_to_storage(token: &str) {
    let _ = document::eval(&format!(
        "window.localStorage.setItem('{TOKEN_STORAGE_KEY}', '{token}');"
    ))
    .await;
}

async fn clear_token_storage() {
    let _ = document::eval(&format!(
        "window.localStorage.removeItem('{TOKEN_STORAGE_KEY}');"
    ))
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_error_strips_server_fn_wrapper() {
        let raw = "error running server function: Usuário ou senha inválidos. (details: None)";
        assert_eq!(friendly_error(raw), "Usuário ou senha inválidos.");
    }

    #[test]
    fn friendly_error_keeps_plain_messages_untouched() {
        assert_eq!(friendly_error("Usuário já existe."), "Usuário já existe.");
    }

    #[test]
    fn friendly_error_falls_back_when_empty() {
        assert_eq!(
            friendly_error("   "),
            "Não foi possível concluir a operação. Tente novamente."
        );
    }
}
