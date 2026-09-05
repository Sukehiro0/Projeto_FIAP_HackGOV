use dioxus::prelude::*;

use crate::accessibility::A11ySettings;
use crate::components::{AccessibilityWidget, ComingSoonDialog, Footer, Header, HelpWidget};
use crate::pages::{Home, MyGovPanel, ServiceDetail};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    #[route("/servicos/:slug")]
    ServiceDetail { slug: String },
    #[route("/minha-vida")]
    MyGovPanel {},
}

#[component]
fn AppLayout() -> Element {
    let a11y = use_context::<A11ySettings>();
    let high_contrast = (a11y.high_contrast)();
    let easy_mode = (a11y.easy_mode)();

    let class = match (high_contrast, easy_mode) {
        (true, true) => "min-h-screen bg-white flex flex-col high-contrast easy-mode",
        (true, false) => "min-h-screen bg-white flex flex-col high-contrast",
        (false, true) => "min-h-screen bg-white flex flex-col easy-mode",
        (false, false) => "min-h-screen bg-white flex flex-col",
    };

    rsx! {
        div {
            class,
            Header {}
            // `id="top"` fica aqui, no wrapper do conteúdo de CADA rota, e não
            // só na Home. Antes, o link de pular conteúdo ("Ir para o
            // conteúdo") e o `#top` só existiam dentro do Hero da Home — em
            // qualquer outra página (ex: /servicos/:slug) o link não levava a
            // lugar nenhum. Agora `#top` sempre existe, em qualquer rota.
            div { id: "top", class: "flex-1", Outlet::<Route> {} }
            Footer {}
            AccessibilityWidget {}
            HelpWidget {}
            ComingSoonDialog {}
        }
    }
}
