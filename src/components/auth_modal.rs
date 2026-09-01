use dioxus::prelude::*;

use crate::auth::AuthState;
use crate::i18n::{t, I18n};

#[derive(Clone, Copy, PartialEq)]
enum AuthMode {
    Login,
    Signup,
}

/// Modal de login/cadastro. `show` controla a visibilidade e é compartilhado
/// com quem abre o modal (normalmente o `Header`).
#[component]
pub fn AuthModal(mut show: Signal<bool>) -> Element {
    let mut auth = use_context::<AuthState>();
    let i18n = use_context::<I18n>();
    let locale = (i18n.locale)();

    let mut mode = use_signal(|| AuthMode::Login);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut submitting = use_signal(|| false);

    let title = if mode() == AuthMode::Login {
        t(locale, "auth.modal_title_login")
    } else {
        t(locale, "auth.modal_title_signup")
    };

    rsx! {
        if show() {
            div { class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/50 px-4",
                div { class: "w-full max-w-sm bg-white rounded-lg shadow-xl p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-lg font-bold text-govbr-blue-dark", "{title}" }
                        button {
                            class: "text-govbr-gray-text hover:text-govbr-blue-dark text-xl leading-none",
                            "aria-label": t(locale, "auth.close"),
                            onclick: move |_| show.set(false),
                            "×"
                        }
                    }

                    form {
                        class: "flex flex-col gap-3",
                        onsubmit: move |e: FormEvent| async move {
                            e.prevent_default();
                            submitting.set(true);
                            let u = username();
                            let p = password();
                            let result = if mode() == AuthMode::Login {
                                auth.login(u, p).await
                            } else {
                                auth.signup(u, p).await
                            };
                            submitting.set(false);
                            if result.is_ok() {
                                show.set(false);
                                username.set(String::new());
                                password.set(String::new());
                            }
                        },
                        label { class: "flex flex-col gap-1 text-sm font-medium text-govbr-blue-dark",
                            "{t(locale, \"auth.username\")}"
                            input {
                                r#type: "text",
                                class: "border border-govbr-gray-border rounded px-3 py-2 text-sm focus:outline-none focus:border-govbr-blue",
                                value: "{username}",
                                oninput: move |e| username.set(e.value()),
                            }
                        }
                        label { class: "flex flex-col gap-1 text-sm font-medium text-govbr-blue-dark",
                            "{t(locale, \"auth.password\")}"
                            input {
                                r#type: "password",
                                class: "border border-govbr-gray-border rounded px-3 py-2 text-sm focus:outline-none focus:border-govbr-blue",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                        }

                        if let Some(err) = (auth.error)() {
                            p { class: "text-xs font-medium text-red-600", "{err}" }
                        }

                        button {
                            r#type: "submit",
                            disabled: submitting(),
                            class: "mt-1 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-4 py-2.5 disabled:opacity-60",
                            if submitting() {
                                {t(locale, "auth.loading")}
                            } else if mode() == AuthMode::Login {
                                {t(locale, "auth.submit_login")}
                            } else {
                                {t(locale, "auth.submit_signup")}
                            }
                        }
                    }

                    button {
                        class: "mt-3 text-xs font-medium text-govbr-blue hover:underline block mx-auto",
                        onclick: move |_| {
                            mode.set(if mode() == AuthMode::Login { AuthMode::Signup } else { AuthMode::Login });
                            auth.error.set(None);
                        },
                        if mode() == AuthMode::Login {
                            {t(locale, "auth.switch_to_signup")}
                        } else {
                            {t(locale, "auth.switch_to_login")}
                        }
                    }

                    p { class: "mt-4 text-[11px] text-govbr-gray-text text-center leading-relaxed",
                        "{t(locale, \"auth.disclaimer\")}"
                    }
                }
            }
        }
    }
}
