use dioxus::prelude::*;

/// Estado de acessibilidade compartilhado por toda a aplicação.
#[derive(Clone, Copy)]
pub struct A11ySettings {
    pub font_scale: Signal<i32>,
    pub high_contrast: Signal<bool>,
}

impl A11ySettings {
    pub fn provide() -> Self {
        let settings = Self {
            font_scale: Signal::new(0),
            high_contrast: Signal::new(false),
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
}

/// Aplica o zoom de fonte no elemento raiz via JS, reagindo a mudanças do signal.
pub fn use_apply_font_scale(font_scale: Signal<i32>) {
    use_effect(move || {
        let pct = 100 + *font_scale.read() * 12;
        document::eval(&format!(
            "document.documentElement.style.fontSize = '{pct}%';"
        ));
    });
}

/// Lê um texto em voz alta usando a Web Speech API do navegador.
pub fn speak(text: &str) {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', " ");
    document::eval(&format!(
        "if ('speechSynthesis' in window) {{
            window.speechSynthesis.cancel();
            const u = new SpeechSynthesisUtterance('{escaped}');
            u.lang = 'pt-BR';
            window.speechSynthesis.speak(u);
        }}"
    ));
}
