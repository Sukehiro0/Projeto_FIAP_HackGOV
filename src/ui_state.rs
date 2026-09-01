use dioxus::prelude::*;

/// Estado de UI compartilhado entre componentes que vivem em pontos diferentes
/// da árvore (rodapé, categorias, widgets flutuantes).
///
/// Existe por um motivo direto: antes, links do rodapé e das categorias sem
/// destino real usavam `href="#"`, o que faz o navegador mostrar a URL como
/// "http://.../#" ao passar o mouse ou clicar — parece um link quebrado.
/// Em vez disso, cada link agora faz uma de duas coisas: (1) se existe uma
/// funcionalidade real equivalente na aplicação (Central de Ajuda,
/// Acessibilidade), o clique abre essa funcionalidade de verdade; (2) se não
/// existe (a maioria dos links institucionais, já que este é um protótipo de
/// hackathon), o clique abre um aviso honesto de simulação — em vez de fingir
/// que existe uma página ali.
#[derive(Clone, Copy)]
pub struct UiPanels {
    pub a11y_open: Signal<bool>,
    pub help_open: Signal<bool>,
    /// Rótulo do link "em simulação" clicado por último, ou `None` se nenhum
    /// aviso deve estar visível.
    pub coming_soon: Signal<Option<&'static str>>,
}

impl UiPanels {
    pub fn provide() -> Self {
        let panels = Self {
            a11y_open: Signal::new(false),
            help_open: Signal::new(false),
            coming_soon: Signal::new(None),
        };
        use_context_provider(|| panels)
    }
}
