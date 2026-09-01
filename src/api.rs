//! Definições das *server functions* de autenticação. Este módulo é sempre
//! compilado (cliente e servidor): no build do servidor, o corpo de cada
//! função vira o handler HTTP real; no build do cliente, o macro `#[post]`
//! gera automaticamente um stub que faz a chamada HTTP equivalente — por
//! isso os corpos abaixo podem chamar `crate::server::*` (só existe quando a
//! feature `server` está ativa) sem nenhum `#[cfg]` manual, exatamente como
//! no exemplo oficial de autenticação do Dioxus.
//!
//! Observação de segurança: usamos POST (em vez de GET) inclusive para
//! `logout`/`me`, para nunca colocar o token de sessão em uma URL/query
//! string (que poderia vazar em logs de servidor, histórico do navegador ou
//! cabeçalhos `Referer`).

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Dados públicos do usuário autenticado (nunca inclui senha ou hash).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub username: String,
}

/// Resposta de cadastro/login bem-sucedido: token de sessão + dados do usuário.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: PublicUser,
}

/// Cria uma nova conta e já efetua o login, devolvendo um token de sessão.
#[post("/api/auth/signup")]
pub async fn signup(username: String, password: String) -> Result<AuthResponse> {
    let clean_username = username.trim().to_string();
    let token = crate::server::create_user(username, password).await?;
    Ok(AuthResponse {
        token,
        user: PublicUser {
            username: clean_username,
        },
    })
}

/// Confere usuário/senha e devolve um novo token de sessão.
#[post("/api/auth/login")]
pub async fn login(username: String, password: String) -> Result<AuthResponse> {
    let clean_username = username.trim().to_string();
    let token = crate::server::login(username, password).await?;
    Ok(AuthResponse {
        token,
        user: PublicUser {
            username: clean_username,
        },
    })
}

/// Invalida a sessão atual.
#[post("/api/auth/logout")]
pub async fn logout(token: String) -> Result<()> {
    crate::server::delete_session(token).await?;
    Ok(())
}

/// Devolve os dados públicos do usuário dono do token, se a sessão ainda for válida.
#[post("/api/auth/me")]
pub async fn me(token: String) -> Result<Option<PublicUser>> {
    let username = crate::server::username_for_token(token).await?;
    Ok(username.map(|username| PublicUser { username }))
}
