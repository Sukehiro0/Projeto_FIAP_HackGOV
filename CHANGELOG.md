# Changelog — revisão de segurança, acessibilidade e identidade visual

Resumo do que mudou nesta rodada, pra você (e seu parceiro de grupo)
saberem exatamente o que foi tocado antes de continuar implementando por
cima. **Não foi possível compilar/testar neste ambiente** (sem toolchain
Rust disponível) — revisão foi feita estaticamente, com bastante cuidado,
mas rode `dx serve` antes de dar commit pra confirmar.

## Identidade visual / ícones
- Novo módulo `src/components/icons.rs`: ~55 ícones SVG desenhados do zero
  (`IconKind` enum + componente `Icon`), sem nenhuma dependência externa.
- `data.rs`: todos os campos `icon` deixaram de ser emoji (`&'static str`)
  e passaram a ser `IconKind`.
- Todos os componentes e páginas que exibiam emoji foram migrados para usar
  `Icon { kind: ..., class: "..." }`.

## Acessibilidade
- Novo `src/components/a11y_widget.rs`: botão flutuante sempre visível
  (bottom-left) com painel de tamanho de fonte, alto contraste e modo
  fácil — substitui a barrinha de texto que sumia no mobile.
- Alvos de toque de pelo menos 44px nos controles de acessibilidade e no
  menu mobile (WCAG 2.5.5).

## Bugs corrigidos
- **Header quebrava fora da Home**: os links `#servicos`, `#categorias`
  etc. agora apontam para `/#servicos`, funcionando em qualquer rota.
- **Menu mobile não fechava** ao clicar num item — corrigido.
- **Botão "Entrar com gov.br" estourava em telas pequenas** — agora vira
  ícone-only abaixo do breakpoint `sm`.

## Segurança (OWASP)
- CSP restritiva via `<meta>` em `src/main.rs`, com as concessões
  necessárias documentadas (`'unsafe-inline'` em `style-src` pela barra de
  risco do FraudShield; `'unsafe-eval'`/`'wasm-unsafe-eval'` em
  `script-src` pelo `document::eval` do Dioxus).
- `Referrer-Policy` via meta.
- Sanitização mais completa da string interpolada em `speak()`
  (`accessibility.rs`), incluindo separadores de linha Unicode.
- `SECURITY.md` novo: checklist OWASP completo, dividido entre "já
  implementado" e "documentado, precisa de backend/infra" (rate limiting,
  HSTS, X-Frame-Options via header real, autenticação real, etc.).

## Comentários
- Adicionado `///` doc comment em todo componente que ainda não tinha.

## Modo fácil agora esconde ruído visual de verdade
- A classe `.easy-mode-hide` existia no CSS desde a primeira rodada, mas
  nenhum componente a usava — o toggle mudava o estado, mas nada
  desaparecia na tela. Agora ela é aplicada na faixa de estatísticas de
  marketing (`Stats`) e na grade de números técnicos do painel de status
  (`StatusPanel`), que são informações de "bom saber" e não afetam a
  conclusão de nenhuma tarefa do cidadão.

## O que ainda vale revisar/fazer (backlog, listado também no SECURITY.md)
- Migrar os dois usos de `document::eval` para bindings tipados, pra poder
  tirar `'unsafe-eval'` do CSP.
- Tirar o `style` inline do `FraudShield` (barra de risco), pra poder tirar
  `'unsafe-inline'` do CSP de `style-src`.
- Definir `lang="pt-BR"` no `<html>` raiz (precisa de `index.html`
  customizado — não mexi nisso agora pra não arriscar quebrar o build sem
  poder testar localmente).
- Rodar `cargo clippy` e `dx serve` antes do commit — este ambiente não
  tem toolchain Rust pra validar a compilação.
