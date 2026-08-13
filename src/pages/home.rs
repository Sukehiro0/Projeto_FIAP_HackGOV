use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "min-h-screen bg-slate-900 text-white flex flex-col items-center justify-center gap-6 p-8",
            h1 { class: "text-4xl font-bold", "🦀 HackGOV" }
            p { class: "text-slate-400", "Front-end em Rust com Dioxus + Tailwind CSS" }
            div { class: "flex items-center gap-4",
                button {
                    class: "w-10 h-10 rounded bg-slate-700 hover:bg-slate-600 transition-colors text-xl",
                    onclick: move |_| count -= 1,
                    "-"
                }
                span { class: "text-2xl font-mono w-12 text-center", "{count}" }
                button {
                    class: "w-10 h-10 rounded bg-indigo-600 hover:bg-indigo-500 transition-colors text-xl",
                    onclick: move |_| count += 1,
                    "+"
                }
            }
        }
    }
}
