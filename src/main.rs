use dioxus::prelude::*;

mod accessibility;
mod api;
mod auth;
mod components;
mod data;
mod i18n;
mod pages;
mod routes;
#[cfg(feature = "server")]
mod server;

use accessibility::{use_apply_font_scale, A11ySettings};
use auth::AuthState;
use i18n::I18n;
use routes::Route;

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
    AuthState::provide();
    I18n::provide();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
