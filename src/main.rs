use dioxus::prelude::*;

mod accessibility;
mod components;
mod data;
mod pages;
mod routes;

use accessibility::{use_apply_font_scale, A11ySettings};
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let a11y = A11ySettings::provide();
    use_apply_font_scale(a11y.font_scale);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
