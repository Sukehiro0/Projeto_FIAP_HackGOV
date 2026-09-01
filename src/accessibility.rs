use dioxus::prelude::*;

/// Estado de acessibilidade compartilhado por toda a aplicação.
#[derive(Clone, Copy)]
pub struct A11ySettings {
    pub font_scale: Signal<i32>,
    pub high_contrast: Signal<bool>,
    /// "Modo fácil": menos informação por tela, mais espaçamento, foco em passo a passo.
    pub easy_mode: Signal<bool>,
}

impl A11ySettings {
    pub fn provide() -> Self {
        let settings = Self {
            font_scale: Signal::new(0),
            high_contrast: Signal::new(false),
            easy_mode: Signal::new(false),
        };
        use_context_provider(|| settings)
    }

    pub fn increase_font(&mut self) {
        let current = *self.font_scale.read();
        self.font_scale.set((current + 1).min(2));
    }

    pub fn decrease_font(&mut self) {
        let current = *self.font_scale.read();
        self.font_scale.set((current - 1).max(-1));
    }

    pub fn reset_font(&mut self) {
        self.font_scale.set(0);
    }

    pub fn toggle_contrast(&mut self) {
        let current = *self.high_contrast.read();
        self.high_contrast.set(!current);
    }

    pub fn toggle_easy_mode(&mut self) {
        let current = *self.easy_mode.read();
        self.easy_mode.set(!current);
    }
}

/// Aplica o zoom de fonte no elemento raiz via JS, reagindo a mudanças do signal.
/// `pct` é sempre um inteiro controlado internamente (88/100/112/124), nunca
/// texto externo, então não há risco de injeção nessa interpolação.
pub fn use_apply_font_scale(font_scale: Signal<i32>) {
    use_effect(move || {
        let pct = 100 + *font_scale.read() * 12;
        document::eval(&format!(
            "document.documentElement.style.fontSize = '{pct}%';"
        ));
    });
}

/// Lê um texto em voz alta usando a Web Speech API do navegador (suporte ao "Modo fácil").
///
/// Segurança: hoje `text` só recebe conteúdo estático de `data.rs` (nunca
/// input do usuário), então o risco de injeção de JavaScript aqui é baixo.
/// Ainda assim, escapamos defensivamente antes de interpolar a string no
/// `document::eval`, incluindo os separadores de linha U+2028/U+2029 (que
/// são válidos dentro de uma string Rust, mas quebram uma string JS entre
/// aspas simples mesmo sem aparecer como `\n` ou `'`). Se este texto um dia
/// passar a vir de uma fonte externa/dinâmica (ex: um campo digitado pelo
/// cidadão), o ideal é parar de montar JS por interpolação de string e
/// passar o valor via `document::eval(...).send(...)` (canal tipado do
/// Dioxus) em vez de `format!` — ver SECURITY.md, item "eval e injeção de JS".
pub fn speak(text: &str) {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\u{2028}', " ")
        .replace('\u{2029}', " ")
        .replace("</", "<\\/");
    document::eval(&format!(
        "if ('speechSynthesis' in window) {{
            window.speechSynthesis.cancel();
            const u = new SpeechSynthesisUtterance('{escaped}');
            u.lang = 'pt-BR';
            window.speechSynthesis.speak(u);
        }}"
    ));
}
