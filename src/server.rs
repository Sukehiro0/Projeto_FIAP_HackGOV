//! Backend real: banco SQLite embutido, hashing de senha (Argon2) e sessões
//! por token opaco.
//!
//! Este módulo só é compilado com a feature `server` (ver `Cargo.toml` e
//! `main.rs`) — nunca entra no bundle WASM enviado ao navegador.
//!
//! Modelo de sessão: para evitar depender de crates de sessão/cookie ainda
//! não validadas neste projeto (ex.: `axum_session_auth`, que exigiria testar
//! compatibilidade de versões do Axum interno do Dioxus), o token de sessão é
//! gerado no cadastro/login e devolvido ao cliente no corpo da resposta, não
//! como cookie. O cliente guarda esse token (ver `src/auth.rs`) e o reenvia
//! como argumento normal nas chamadas que exigem login.
//!
//! Isso é suficiente para uma demonstração, mas — diferente de um cookie
//! `HttpOnly` — o token fica acessível a JavaScript no navegador (maior
//! superfície para roubo via XSS). Para um sistema em produção real, o
//! recomendado seria migrar para cookies assinados/`HttpOnly` seguindo o
//! guia oficial do Dioxus Fullstack sobre autenticação.
//!
//! Outras proteções aplicadas:
//! - Senhas nunca são guardadas em texto puro: usamos Argon2 (vencedor da
//!   Password Hashing Competition, recomendado pela OWASP).
//! - O token de sessão em si também não é guardado em texto puro no banco:
//!   guardamos apenas o SHA-256 dele, então um vazamento do banco não expõe
//!   sessões válidas diretamente.
//! - Sessões expiram (7 dias) e o logout apaga a sessão do banco.

use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use rand_core::{OsRng, RngCore};
use rusqlite::{Connection, OptionalExtension};
use sha2::{Digest, Sha256};

const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7; // 7 dias

fn db_path() -> String {
    std::env::var("HACKGOV_DB_PATH").unwrap_or_else(|_| "hackgov.db".to_string())
}

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

fn connection() -> &'static Mutex<Connection> {
    DB.get_or_init(|| {
        let conn = Connection::open(db_path()).expect("falha ao abrir o banco SQLite");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sessions (
                token_hash TEXT PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                expires_at INTEGER NOT NULL
            );",
        )
        .expect("falha ao criar as tabelas do banco");
        Mutex::new(conn)
    })
}

/// Executa uma função bloqueante de acesso ao banco em uma thread dedicada
/// (`spawn_blocking`), para não travar o executor assíncrono do servidor.
async fn with_db<T, F>(f: F) -> anyhow::Result<T>
where
    F: FnOnce(&Connection) -> anyhow::Result<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let conn = connection()
            .lock()
            .map_err(|_| anyhow::anyhow!("mutex do banco de dados envenenado"))?;
        f(&conn)
    })
    .await
    .map_err(|e| anyhow::anyhow!("tarefa do banco de dados falhou: {e}"))?
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("falha ao gerar hash da senha: {e}"))?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    let parsed =
        PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("hash de senha inválido: {e}"))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn valid_username(username: &str) -> bool {
    let len_ok = (3..=32).contains(&username.chars().count());
    let chars_ok = username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    len_ok && chars_ok
}

/// Cria um novo usuário e já devolve uma sessão (login automático após
/// cadastro). Retorna o nome de usuário normalizado e o token de sessão.
pub async fn create_user(username: String, password: String) -> anyhow::Result<String> {
    let username = username.trim().to_string();
    if !valid_username(&username) {
        anyhow::bail!(
            "Nome de usuário inválido. Use de 3 a 32 letras, números, ponto ou sublinhado."
        );
    }
    if password.chars().count() < 8 {
        anyhow::bail!("A senha deve ter pelo menos 8 caracteres.");
    }

    let password_hash = hash_password(&password)?;
    let created_at = now_secs();
    let uname = username.clone();

    let user_id = with_db(move |conn| {
        let existing: Option<i64> = conn
            .query_row(
                "SELECT id FROM users WHERE username = ?1",
                [&uname],
                |row| row.get(0),
            )
            .optional()?;
        if existing.is_some() {
            anyhow::bail!("Esse nome de usuário já está em uso.");
        }
        conn.execute(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![uname, password_hash, created_at],
        )?;
        Ok(conn.last_insert_rowid())
    })
    .await?;

    create_session(user_id).await
}

/// Confere usuário/senha e, se válidos, cria e devolve um novo token de sessão.
pub async fn login(username: String, password: String) -> anyhow::Result<String> {
    let uname = username.trim().to_string();
    let row = with_db(move |conn| {
        let row: Option<(i64, String)> = conn
            .query_row(
                "SELECT id, password_hash FROM users WHERE username = ?1",
                [&uname],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        Ok(row)
    })
    .await?;

    let (user_id, password_hash) =
        row.ok_or_else(|| anyhow::anyhow!("Usuário ou senha inválidos."))?;

    if !verify_password(&password, &password_hash)? {
        anyhow::bail!("Usuário ou senha inválidos.");
    }

    create_session(user_id).await
}

/// Cria uma sessão nova para o usuário e devolve o token (não hasheado) que
/// deve ser guardado pelo cliente.
async fn create_session(user_id: i64) -> anyhow::Result<String> {
    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at = now_secs() + SESSION_TTL_SECS;

    with_db(move |conn| {
        conn.execute(
            "INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![token_hash, user_id, expires_at],
        )?;
        Ok(())
    })
    .await?;

    Ok(token)
}

/// Devolve o nome de usuário associado a um token de sessão válido (e ainda
/// não expirado), se existir.
pub async fn username_for_token(token: String) -> anyhow::Result<Option<String>> {
    let token_hash = hash_token(&token);
    let now = now_secs();

    with_db(move |conn| {
        let row: Option<String> = conn
            .query_row(
                "SELECT users.username FROM sessions
                 JOIN users ON users.id = sessions.user_id
                 WHERE sessions.token_hash = ?1 AND sessions.expires_at > ?2",
                rusqlite::params![token_hash, now],
                |row| row.get(0),
            )
            .optional()?;
        Ok(row)
    })
    .await
}

/// Invalida uma sessão (logout). Não falha se o token já não existir.
pub async fn delete_session(token: String) -> anyhow::Result<()> {
    let token_hash = hash_token(&token);
    with_db(move |conn| {
        conn.execute("DELETE FROM sessions WHERE token_hash = ?1", [&token_hash])?;
        Ok(())
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_username_accepts_reasonable_names() {
        assert!(valid_username("joao.silva"));
        assert!(valid_username("ana_2024"));
        assert!(valid_username("abc"));
    }

    #[test]
    fn valid_username_rejects_bad_names() {
        assert!(!valid_username("ab")); // curto demais
        assert!(!valid_username(&"a".repeat(33))); // longo demais
        assert!(!valid_username("tem espaço"));
        assert!(!valid_username("tem@arroba"));
    }

    #[test]
    fn password_hash_round_trip() {
        let hash = hash_password("uma-senha-bem-forte").expect("deve gerar hash");
        assert!(verify_password("uma-senha-bem-forte", &hash).unwrap());
        assert!(!verify_password("senha-errada", &hash).unwrap());
    }

    #[test]
    fn password_hashes_are_salted_differently() {
        let a = hash_password("mesma-senha").unwrap();
        let b = hash_password("mesma-senha").unwrap();
        assert_ne!(a, b, "cada hash deve usar um salt diferente");
    }

    #[test]
    fn generate_token_is_random_and_hex_encoded() {
        let a = generate_token();
        let b = generate_token();
        assert_ne!(a, b);
        assert_eq!(a.len(), 64); // 32 bytes em hex
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_token_is_deterministic_and_not_reversible_looking() {
        let token = generate_token();
        assert_eq!(hash_token(&token), hash_token(&token));
        assert_ne!(hash_token(&token), token);
    }
}
