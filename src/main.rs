use dioxus::prelude::*;

mod accessibility;
mod api;
mod auth;
mod components;
mod data;
mod i18n;
mod pages;
mod routes;
<<<<<<< HEAD
mod ui_state;
=======
#[cfg(feature = "server")]
mod server;
>>>>>>> c09a621e3ecc8da34ca00dd2db84b30738ee7099

use accessibility::{use_apply_font_scale, A11ySettings};
use auth::AuthState;
use i18n::I18n;
use routes::Route;
use ui_state::UiPanels;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // No cliente (web/desktop/mobile), apenas iniciamos o app normalmente.
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    // No servidor, criamos um router Axum customizado a partir do app.
    // O `dioxus::server::router` já cuida de servir os assets estáticos,
    // renderizar o app em HTML (SSR) e registrar as server functions.
    #[cfg(feature = "server")]
    dioxus::serve(|| async move { Ok(dioxus::server::router(App)) });
}

#[component]
fn App() -> Element {
    let a11y = A11ySettings::provide();
    use_apply_font_scale(a11y.font_scale);
<<<<<<< HEAD
    UiPanels::provide();
=======
    AuthState::provide();
    I18n::provide();
>>>>>>> c09a621e3ecc8da34ca00dd2db84b30738ee7099

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        // --- Cabeçalhos de segurança aplicados via <meta> (defesa em profundidade) ---
        //
        // Isto é um SPA 100% estático (compilado para WASM, sem backend próprio),
        // então o CSP abaixo é propositalmente restritivo: nada é buscado de
        // fora do próprio domínio, então praticamente tudo pode ser 'self'.
        //
        // IMPORTANTE: alguns cabeçalhos de segurança (X-Frame-Options,
        // Strict-Transport-Security, X-Content-Type-Options, e a diretiva
        // `frame-ancestors` do próprio CSP) só têm efeito quando enviados como
        // cabeçalho HTTP real pelo servidor/CDN que hospeda os arquivos — o
        // navegador ignora esses casos quando vêm de <meta>. Isso está
        // documentado com o restante do checklist OWASP em SECURITY.md.
        // Observação sobre `style-src`: o card de segurança da conta usa um
        // atributo `style` inline para desenhar a barra de risco com largura
        // dinâmica (0–100%), então `'unsafe-inline'` é necessário aqui. O
        // risco real disso é baixo (CSS inline não executa JavaScript), mas
        // documentamos como uma concessão consciente — o ideal seria migrar
        // essa barra para JS mínimo controlado por nonce/hash, ou embutir a
        // largura como uma custom property setada por um único trecho de
        // script confiável. Ver SECURITY.md, item "CSP — style-src".
        // Observação sobre `script-src`: a aplicação usa `document::eval`
        // em dois pontos (accessibility.rs::speak, e o observer de scroll
        // em home.rs) para conversar com APIs do navegador (SpeechSynthesis,
        // IntersectionObserver) que o Dioxus ainda não expõe como hooks
        // nativos. Esse mecanismo do Dioxus depende de avaliação de string
        // como JS no navegador, o que exige 'unsafe-eval' — não dá pra
        // simplesmente remover sem reescrever essas duas chamadas usando
        // bindings wasm-bindgen tipados (item documentado como "difícil de
        // implementar agora" em SECURITY.md, seção "CSP — script-src").
        // Como mitigação, os únicos textos passados para `speak()` vêm de
        // `data.rs` (conteúdo estático nosso, nunca de input do usuário).
        document::Meta {
            http_equiv: "Content-Security-Policy",
            content: "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'self'; upgrade-insecure-requests",
        }
        document::Meta { name: "referrer", content: "strict-origin-when-cross-origin" }
        document::Meta { name: "description", content: "hack.gov.br — simulação de portal de serviços públicos digitais, projeto de demonstração (HackGOV)." }

        Router::<Route> {}
    }
}
