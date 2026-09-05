# HackGOV — Front-end em Rust (Dioxus + Tailwind CSS)

Projeto front-end escrito em Rust, compilado para WebAssembly, usando o framework
[Dioxus](https://dioxuslabs.com/) (v0.7) com [Tailwind CSS](https://tailwindcss.com/) (v4)
para estilização.

Além do código, este README funciona como uma **aula de Rust e Dioxus**: depois das
instruções de instalação, ele explica os conceitos de linguagem/framework usados aqui e
faz um passeio arquivo por arquivo por todo o projeto.

## Sumário

- [O que é o HackGOV](#o-que-é-o-hackgov)
- [Pré-requisitos já instalados](#pré-requisitos-já-instalados)
- [Rodando em desenvolvimento](#rodando-em-desenvolvimento)
- [Build de produção](#build-de-produção)
- [Testes, lint e CI/CD](#testes-lint-e-cicd)
- [Estrutura do projeto](#estrutura-do-projeto)
- [Escrevendo componentes](#escrevendo-componentes)
- [Aula de Rust: conceitos usados neste projeto](#aula-de-rust-conceitos-usados-neste-projeto)
- [Aula de Dioxus: conceitos usados neste projeto](#aula-de-dioxus-conceitos-usados-neste-projeto)
- [Passeio guiado pelo código, arquivo por arquivo](#passeio-guiado-pelo-código-arquivo-por-arquivo)
- [Glossário rápido](#glossário-rápido)
- [Problema conhecido nesta máquina: Smart App Control](#problema-conhecido-nesta-máquina-smart-app-control)
- [Licença](#licença)

## O que é o HackGOV

HackGOV é uma landing page **fictícia** inspirada no [gov.br](https://www.gov.br/pt-br),
com uma proposta mais tecnológica e transparente. **Não é um site oficial do governo**
(o aviso fica sempre visível no rodapé) — é um projeto de demonstração/hackathon para
explorar, em Rust, como um portal de serviços públicos poderia:

- Deixar mais fácil achar um serviço: busca com autocomplete
  ([src/components/hero.rs](src/components/hero.rs)) e um "assistente" que entende
  frases livres tipo *"perdi minha carteira de motorista"*
  ([src/components/assistant.rs](src/components/assistant.rs)).
- Mostrar tudo sobre o cidadão em um único painel, em vez de espalhado por sistemas
  diferentes ([src/pages/my_gov.rs](src/pages/my_gov.rs)).
- Ser transparente sobre desempenho e segurança — nota média, tempo de espera e
  principal reclamação de cada serviço
  ([src/components/performance.rs](src/components/performance.rs)), uptime da
  plataforma ([src/components/status.rs](src/components/status.rs)) e uma simulação de
  sistema antifraude com "defesa em profundidade"
  ([src/components/fraud_shield.rs](src/components/fraud_shield.rs)).
- Ser acessível de verdade: zoom de fonte, alto contraste e leitura em voz
  alta ([src/accessibility.rs](src/accessibility.rs)).
- Simular, com estado local (sem backend real), o fluxo completo de pedir um serviço:
  checklist prévio → confirmação → verificação facial com alternativa em caso de falha →
  acompanhamento passo a passo → feedback
  ([src/pages/service_detail.rs](src/pages/service_detail.rs)).

Todo o app roda **inteiramente no navegador** (Dioxus compilado para WebAssembly, feature
`web`), sem servidor/backend: os "dados" (serviços, categorias, alertas, postos de
atendimento etc.) são só funções Rust que devolvem `Vec` fixos, em
[src/data.rs](src/data.rs). Não existe login real nem persistência — é tudo simulado
para focar no front-end.

## Pré-requisitos já instalados

- Rust (`rustc`, `cargo`) via `rustup`
- Target `wasm32-unknown-unknown`
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/guides/tools/) (`dx`)
- Visual Studio Build Tools (linker MSVC, necessário no Windows)
- Node.js + npm (usado apenas para o Tailwind CLI)

Se abrir este projeto em outra máquina, garanta que o PATH inclui `%USERPROFILE%\.cargo\bin`
(normalmente já configurado automaticamente por novos terminais depois da instalação do Rust).

## Rodando em desenvolvimento

Em um terminal, rode o watcher do Tailwind (gera `assets/tailwind.css` a partir de
`tailwind.css` sempre que uma classe nova for usada):

```powershell
npm run watch:css
```

Em outro terminal, sirva a aplicação com hot-reload:

```powershell
dx serve
```

Isso abre um servidor local (normalmente em `http://127.0.0.1:8080`) e recompila
automaticamente a cada alteração no código Rust.

> Nota: o próprio `dx serve` consegue instalar/gerenciar um binário standalone do
> Tailwind automaticamente na primeira execução. O `npm run watch:css` acima é uma
> alternativa equivalente usando o pacote `@tailwindcss/cli` já instalado localmente.

## Build de produção

```powershell
dx bundle
npm run build:css
```

Os artefatos ficam em `target/dx/hackgov/release/web/`.

## Testes, lint e CI/CD

```powershell
cargo test                                  # testes unitários (lógica pura em src/data.rs)
cargo clippy --all-targets -- -D warnings   # lint, tratando warnings como erro
cargo fmt --all -- --check                  # verifica formatação sem alterar arquivos
```

Os testes cobrem principalmente [src/data.rs](src/data.rs): faixas de decisão do
`fraud_level()`, invariantes dos dados mock (slugs únicos, listas não vazias) e o
reconhecimento por palavras-chave do assistente (`match_services()`, usado por
[src/components/assistant.rs](src/components/assistant.rs)) — incluindo um teste de
regressão para keywords curtas não "vazarem" para dentro de palavras não relacionadas
(ex.: "ir" dentro de "carte**ir**a").

O GitHub Actions roda esses três comandos, mais o build do Tailwind CSS, a cada push/PR
para `main` ([.github/workflows/ci.yml](.github/workflows/ci.yml)). Um segundo workflow
([.github/workflows/deploy.yml](.github/workflows/deploy.yml)) publica automaticamente a
versão web (`dx bundle --release`) no GitHub Pages a cada push em `main` — é necessário
habilitar uma vez, em Settings → Pages → Source, a opção **GitHub Actions** no repositório.

## Estrutura do projeto

```
├─ assets/                 # Ícones, favicon e o CSS compilado (tailwind.css)
├─ src/
│  ├─ main.rs              # Ponto de entrada: cria o contexto de acessibilidade e monta o Router
│  ├─ routes.rs            # enum Route (rotas) + AppLayout (Header/Outlet/Footer/HelpWidget)
│  ├─ accessibility.rs     # A11ySettings (fonte, alto contraste) via Context API
│  ├─ data.rs              # "Banco de dados" fake: services(), categories(), alerts()...
│  ├─ components/          # Um arquivo por seção/widget de UI reutilizável
│  │  ├─ header.rs         # Barra de acessibilidade + cabeçalho com menu
│  │  ├─ hero.rs           # Busca com autocomplete na home
│  │  ├─ assistant.rs      # "Pergunte ao assistente" (busca por linguagem natural)
│  │  ├─ stats.rs          # Números institucionais (5.6 mil+ serviços, etc.)
│  │  ├─ services.rs       # Lista numerada "Serviços para você"
│  │  ├─ categories.rs     # Grade "Navegue por categoria"
│  │  ├─ security.rs       # Cards "Sua segurança é nossa prioridade"
│  │  ├─ status.rs         # Painel de transparência/uptime da plataforma
│  │  ├─ performance.rs    # PerformanceCard: nota, tempo médio e reclamações de um serviço
│  │  ├─ fraud_shield.rs   # FraudShield: score antifraude simulado
│  │  ├─ alerts.rs         # AlertsBell: sino de notificações no cabeçalho
│  │  ├─ help_widget.rs    # Botão flutuante "Não consigo resolver"
│  │  ├─ feedback.rs       # Formulário de avaliação ao final de um serviço
│  │  ├─ footer.rs         # Rodapé institucional
│  │  └─ mod.rs            # Reexporta todos os componentes acima
│  └─ pages/               # Uma página por rota
│     ├─ home.rs           # Página inicial: junta Hero, Assistant, Stats, Services...
│     ├─ service_detail.rs # /servicos/:slug — máquina de estados do fluxo de um serviço
│     ├─ my_gov.rs         # /minha-vida — painel único do cidadão
│     └─ mod.rs            # Reexporta Home, ServiceDetail, MyGovPanel
├─ tailwind.css            # Entrada do Tailwind (@import "tailwindcss"; + tema @theme)
├─ Cargo.toml              # Dependências e features do Rust
├─ Dioxus.toml             # Configuração do Dioxus CLI (título, assets, etc.)
├─ package.json            # Apenas as dependências de build do Tailwind CSS
├─ AGENTS.md               # Referência rápida da API do Dioxus 0.7 (útil para o Copilot)
└─ SETUP.md                # Passo a passo de como montar o ambiente do zero
```

## Escrevendo componentes

Componentes Dioxus são funções Rust anotadas com `#[component]` que retornam `Element`,
usando a macro `rsx!` (sintaxe parecida com JSX) e classes Tailwind no atributo `class`:

```rust
#[component]
fn Botao(texto: String) -> Element {
    rsx! {
        button { class: "px-4 py-2 rounded bg-indigo-600 hover:bg-indigo-500 text-white",
            "{texto}"
        }
    }
}
```

Veja mais exemplos e padrões (signals, contexto, async, roteamento) em
[AGENTS.md](AGENTS.md) ou na [documentação oficial](https://dioxuslabs.com/learn/0.7/).

## Aula de Rust: conceitos usados neste projeto

Rust tem fama de "difícil", mas boa parte disso é o compilador *obrigando* você a lidar
com problemas (nulos, tipos incompatíveis, casos esquecidos) que em outras linguagens só
aparecem em produção. Esta seção usa código real do projeto para explicar as construções
mais importantes.

### Módulos: `mod`, `pub` e `use`

Rust organiza código em módulos, um por arquivo (ou pasta com `mod.rs`). Em
[src/main.rs](src/main.rs):

```rust
mod accessibility;
mod components;
mod data;
mod pages;
mod routes;
```

Cada `mod nome;` diz "existe um arquivo `nome.rs` (ou pasta `nome/mod.rs`) fazendo parte
deste crate". Por padrão, tudo em Rust é **privado** ao módulo onde foi declarado — é
preciso marcar com `pub` para expor algo para fora. Em
[src/components/mod.rs](src/components/mod.rs):

```rust
mod header;
pub use header::Header;
```

`mod header;` só declara o submódulo (privado); `pub use header::Header;` reexporta o
tipo `Header` na raiz de `components`, permitindo que qualquer outro arquivo escreva
`use crate::components::Header;` em vez do caminho completo
`crate::components::header::Header`. É o mesmo padrão usado em
[src/pages/mod.rs](src/pages/mod.rs) para `Home`, `ServiceDetail` e `MyGovPanel`.

### Structs: modelando dados

Um `struct` agrupa campos nomeados. O `Service` em [src/data.rs](src/data.rs) é o
melhor exemplo — modela tudo que uma linha de "serviço" precisa:

```rust
#[derive(Clone, PartialEq)]
pub struct Service {
    pub slug: &'static str,
    pub name: &'static str,
    pub time_estimate: &'static str,
    pub keywords: &'static [&'static str],
    pub requirements: &'static [&'static str],
    pub needs_biometrics: bool,
    pub rating: f32,
    // ...
}
```

- `&'static str` é uma *fatia de string* (slice) com tempo de vida (`lifetime`)
  `'static`: o texto está gravado direto no binário compilado, nunca é alocado no heap
  nem precisa ser liberado. Ótimo para dados fixos como estes — se o texto viesse de
  input do usuário ou de uma API, o tipo certo seria `String` (dona dos próprios bytes,
  alocada no heap).
- `&'static [&'static str]` é uma fatia de fatias: uma lista fixa de strings, como um
  array que nunca muda de tamanho (usado em `keywords`/`requirements`).
- `#[derive(Clone, PartialEq)]` gera automaticamente a implementação de duas *traits*
  (interfaces): `Clone` (permite `.clone()`, cópia explícita) e `PartialEq` (permite
  comparar com `==`). Dioxus **exige** que toda prop de componente implemente essas duas
  traits, porque é assim que ele decide se precisa re-renderizar quando as props mudam.

### Enums e `match`: representando estados finitos

Um `enum` representa "um valor entre um conjunto fixo de opções". Diferente de usar
strings soltas (`"urgent"`, `"warning"`...), o compilador garante que você nunca digitou
errado nem esqueceu um caso. Exemplo em [src/data.rs](src/data.rs):

```rust
#[derive(Clone, Copy, PartialEq)]
pub enum AlertLevel {
    Urgent,
    Warning,
    Info,
}
```

E o uso com `match` em [src/components/alerts.rs](src/components/alerts.rs), onde cada
variante decide uma classe CSS diferente:

```rust
class: match a.level {
    AlertLevel::Urgent => "shrink-0 w-2 h-2 rounded-full bg-red-500 mt-2",
    AlertLevel::Warning => "shrink-0 w-2 h-2 rounded-full bg-govbr-yellow mt-2",
    AlertLevel::Info => "shrink-0 w-2 h-2 rounded-full bg-govbr-blue mt-2",
},
```

Se amanhã alguém adicionar uma variante nova em `AlertLevel` (por exemplo `Critical`) e
esquecer de atualizar este `match`, **o código não compila** — Rust obriga `match` a ser
exaustivo (cobrir todos os casos, a menos que exista um braço `_ => ...` coringa). Essa
garantia elimina uma classe inteira de bugs de "esqueci de tratar esse caso", comuns em
`switch` de outras linguagens.

O mesmo padrão aparece em [src/data.rs](src/data.rs) classificando um score numérico em
uma faixa de risco (repare que os braços de `match` em Rust podem ser intervalos
numéricos):

```rust
pub fn fraud_level(score: i32) -> FraudRiskLevel {
    match score {
        0..=30 => FraudRiskLevel::Normal,
        31..=60 => FraudRiskLevel::Monitoring,
        61..=80 => FraudRiskLevel::Verification,
        _ => FraudRiskLevel::Blocked,
    }
}
```

### `Option<T>`: ausência de valor sem `null`

Rust não tem `null`/`nil`. Quando um valor pode não existir, o tipo é `Option<T>` — que
só pode ser `Some(valor)` ou `None`. Em
[src/pages/service_detail.rs](src/pages/service_detail.rs), a busca por um serviço pelo
slug da URL pode não achar nada:

```rust
let service = services().into_iter().find(|s| s.slug == slug); // Option<Service>
```

E o componente trata os dois casos com `if let ... else`:

```rust
if let Some(s) = service {
    // renderiza a página do serviço
} else {
    // renderiza "Serviço não encontrado"
}
```

O compilador **não deixa** você usar `s` como se fosse sempre um `Service` — você é
obrigado a lidar com o caso `None` em algum momento, o que elimina o clássico erro de
"null pointer" / "undefined is not a function". O mesmo padrão aparece em
`LifeArea.related_slug: Option<&'static str>` ([src/data.rs](src/data.rs)) e no
`if let Some(slug) = area.related_slug { ... }` em
[src/pages/my_gov.rs](src/pages/my_gov.rs).

### Iteradores e closures

Praticamente todo dado exibido na tela passa por uma cadeia de métodos de iterador. Em
[src/components/assistant.rs](src/components/assistant.rs), o campo de busca livre filtra
os serviços comparando as `keywords` de cada um com as palavras digitadas:

```rust
let words: Vec<&str> = q.split_whitespace().collect();
services()
    .into_iter()
    .filter(|s| {
        s.keywords.iter().any(|k| {
            if k.contains(' ') {
                q.contains(k)
            } else {
                words.contains(k)
            }
        })
    })
    .take(3)
    .collect::<Vec<_>>()
```

- `.into_iter()` consome o `Vec<Service>` e itera dono dos valores; `.iter()` (usado em
  `s.keywords.iter()`) itera por referência, sem tomar posse.
- `.filter(|s| ...)` recebe uma **closure** (função anônima que pode capturar variáveis
  do escopo, aqui `q` e `words`) e mantém só os itens em que ela retorna `true`.
  `.any(|k| ...)` retorna `true` assim que algum item satisfaz a condição. `.take(3)`
  para depois de 3 itens. `.collect::<Vec<_>>()` junta tudo de volta em um `Vec`.
- Repare no `if k.contains(' ') { ... } else { ... }`: para uma keyword de uma palavra só
  (ex.: `"ir"`), comparar com `.contains()` bateria por engano dentro de
  "carte**ir**a" — por isso keywords de uma palavra exigem correspondência exata
  (`words.contains(k)`), e só frases de várias palavras usam substring
  (`q.contains(k)`). Um pequeno exemplo real de como o "óbvio" (`.contains()`) pode
  esconder um bug sutil.
- Closures em manipuladores de evento usam a palavra-chave `move`, ex.
  `onclick: move |_| open.toggle()` — `move` força a closure a **tomar posse** das
  variáveis capturadas (aqui, o `Signal` `open`) em vez de só emprestar uma referência.
  Isso é necessário porque o event handler pode ser chamado bem depois de a função do
  componente já ter retornado; `Signal<T>` foi projetado para ser barato de copiar
  (`Copy`) exatamente para tornar esse padrão seguro e simples.

Outros métodos de iterador usados no projeto: `.enumerate()` (índice + item, para
numerar a lista em [src/components/services.rs](src/components/services.rs)), `.map()`
(transformar cada item), `.find()` (primeiro item que bate a condição), `.copied()`
(copiar itens de uma iteração por referência para itens por valor, usado em
`s.requirements.iter().copied()` em
[src/pages/service_detail.rs](src/pages/service_detail.rs)).

### "Banco de dados" fake com funções puras

Não existe backend neste projeto — [src/data.rs](src/data.rs) só tem funções que
retornam `Vec<T>` fixos (`services()`, `categories()`, `alerts()`, `life_areas()`,
`attendance_points()`, `help_reasons()`, `fraud_signals()`). Cada chamada reconstrói a
lista do zero (nada é persistido); é uma forma simples de simular uma fonte de dados
única sem precisar de banco de dados real, API ou arquivo — de propósito, para manter o
projeto 100% front-end.

## Aula de Dioxus: conceitos usados neste projeto

[Dioxus](https://dioxuslabs.com/) é um framework de UI em Rust parecido com React: você
escreve **componentes** (funções que retornam a árvore de UI) que reagem a mudanças de
**estado** (signals), e o framework recalcula só a parte da tela que precisa mudar.

### Componentes e `rsx!`

Um componente é uma função anotada com `#[component]` que devolve `Element`. Dentro dela,
a macro `rsx!` descreve a árvore de elementos com uma sintaxe parecida com HTML/JSX. Veja
[src/components/stats.rs](src/components/stats.rs), um dos componentes mais simples do
projeto:

```rust
#[component]
pub fn Stats() -> Element {
    let stats = [ /* ... */ ];
    rsx! {
        section { class: "reveal border-b border-govbr-gray-border bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                div { class: "grid grid-cols-2 md:grid-cols-4 gap-8",
                    for stat in stats {
                        div { class: "text-center",
                            p { class: "text-3xl sm:text-4xl font-extrabold text-govbr-blue", "{stat.value}" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "{stat.label}" }
                        }
                    }
                }
            }
        }
    }
}
```

Repare que dentro de `rsx!` dá para usar `for`/`if` diretamente, sem `.map()` — o Dioxus
trata isso como controle de fluxo especial da macro (ver [AGENTS.md](AGENTS.md)).
`"{stat.value}"` é interpolação de string: em qualquer texto dentro de `rsx!`,
`{expressão}` insere o valor formatado.

### `Signal<T>`: estado local

`use_signal` cria estado que, quando muda, faz o componente re-renderizar. Em
[src/components/alerts.rs](src/components/alerts.rs):

```rust
let mut open = use_signal(|| false);
// ...
onclick: move |_| open.toggle(),
// ...
if open() { /* mostra o dropdown */ }
```

`use_signal` recebe uma closure que produz o valor **inicial** (só roda uma vez).
Chamar `open()` como se fosse uma função lê (e clona) o valor atual; `.set(valor)`
substitui o valor; `.toggle()` é um atalho para inverter um `bool`; `.write()`/`.read()`
dão acesso mutável/imutável direto ao valor (usados em `A11ySettings`, em
[src/accessibility.rs](src/accessibility.rs)).

### `use_memo`: valores derivados e cacheados

Quando um valor é *calculado a partir de* um signal, `use_memo` evita recalcular a cada
render — só recalcula quando os signals lidos dentro dele mudam. Em
[src/components/hero.rs](src/components/hero.rs), os resultados da busca só são
recalculados quando o texto digitado (`query`) muda:

```rust
let results = use_memo(move || {
    let q = query().trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    services()
        .into_iter()
        .filter(|s| s.name.to_lowercase().contains(&q) || s.tag.to_lowercase().contains(&q))
        .take(5)
        .collect::<Vec<_>>()
});
```

O mesmo padrão aparece no `matches` de
[src/components/assistant.rs](src/components/assistant.rs).

### `use_effect`: efeitos colaterais e interoperabilidade com JavaScript

`use_effect` roda uma closure depois da renderização, e de novo sempre que um signal lido
dentro dela mudar — é o lugar certo para chamar APIs do navegador que o Dioxus não
encapsula diretamente, via `document::eval` (executa uma string de JavaScript no
navegador). Dois usos reais no projeto:

1. Zoom de fonte, em [src/accessibility.rs](src/accessibility.rs) — roda de novo toda
   vez que `font_scale` muda, porque a closure **lê** o signal:

   ```rust
   pub fn use_apply_font_scale(font_scale: Signal<i32>) {
       use_effect(move || {
           let pct = 100 + *font_scale.read() * 12;
           document::eval(&format!(
               "document.documentElement.style.fontSize = '{pct}%';"
           ));
       });
   }
   ```

2. Animação de "fade-in ao rolar a página", em [src/pages/home.rs](src/pages/home.rs) —
   registra um `IntersectionObserver` do JavaScript assim que a Home monta (não lê
   nenhum signal, então roda só uma vez):

   ```rust
   use_effect(|| {
       document::eval(r#"
           setTimeout(() => {
               const obs = new IntersectionObserver((entries) => { /* ... */ });
               document.querySelectorAll('.reveal').forEach((el) => obs.observe(el));
           }, 50);
       "#);
   });
   ```

A mesma ideia de "escape hatch" para JavaScript aparece em `speak()`
([src/accessibility.rs](src/accessibility.rs)), que usa a Web Speech API do navegador
para ler um texto em voz alta (botão "🔊 Ouvir" no detalhe de serviço).

### Context API: estado global sem *prop drilling*

Quando muitos componentes espalhados pela árvore precisam do mesmo estado (aqui, as
configurações de acessibilidade), passar como prop por vários níveis seria repetitivo. O
Context API resolve isso: um componente ancestral "provê" um valor, e qualquer
descendente "consome" o mesmo valor diretamente. Em [src/main.rs](src/main.rs):

```rust
let a11y = A11ySettings::provide(); // por baixo dos panos: use_context_provider(|| settings)
```

E em qualquer componente descendente, como
[src/components/header.rs](src/components/header.rs) e
[src/routes.rs](src/routes.rs):

```rust
let mut a11y = use_context::<A11ySettings>();
```

Isso funciona porque `A11ySettings` (definido em
[src/accessibility.rs](src/accessibility.rs)) é `#[derive(Clone, Copy)]` e só guarda
`Signal<T>`s — copiar a struct copia referências leves para o mesmo estado
compartilhado, não os dados em si.

### Props e componentes com parâmetros

Um componente pode receber parâmetros como argumentos de função (as "props"). Em
[src/components/performance.rs](src/components/performance.rs):

```rust
#[component]
pub fn PerformanceCard(service: Service) -> Element { /* ... */ }
```

usado como `PerformanceCard { service: s.clone() }` em
[src/pages/service_detail.rs](src/pages/service_detail.rs) — por isso `Service` precisa
implementar `Clone` (para o `.clone()`) e `PartialEq` (para o Dioxus comparar props entre
renders e decidir se precisa atualizar o componente).

### Roteamento: `Routable`, `#[route]`, `#[layout]` e `Outlet`

Todas as rotas do app ficam em um único `enum` ([src/routes.rs](src/routes.rs)):

```rust
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
```

- `:slug` é um **segmento dinâmico**: qualquer parte da URL nessa posição vira o campo
  `slug: String` da variante — e o Dioxus passa esse valor automaticamente como prop
  para o componente `ServiceDetail(slug: String)` em
  [src/pages/service_detail.rs](src/pages/service_detail.rs). Não existe *parsing*
  manual de URL em nenhum lugar do código.
- `#[layout(AppLayout)]` diz que `Home`, `ServiceDetail` e `MyGovPanel` compartilham o
  mesmo layout. `AppLayout` renderiza `Header`, depois `Outlet::<Route> {}` (onde a
  página da rota atual "encaixa"), depois `Footer` e `HelpWidget` — assim o cabeçalho, o
  rodapé e o botão flutuante de ajuda aparecem em todas as páginas sem repetir código.
- `Link { to: Route::ServiceDetail { slug: s.slug.to_string() }, ... }` (usado em
  [src/components/hero.rs](src/components/hero.rs),
  [src/components/services.rs](src/components/services.rs) etc.) navega sem recarregar a
  página, e é **verificado em tempo de compilação**: se a rota não existisse, o código
  simplesmente não compilaria — diferente de um `<a href="/algum/lugar">` com string
  solta, que só quebra em tempo de execução.

### Assets e CSS

`asset!("/assets/favicon.ico")` (em [src/main.rs](src/main.rs)) resolve o caminho do
arquivo em tempo de compilação e devolve um identificador que o Dioxus sabe empacotar.
`document::Link`/`document::Stylesheet` injetam tags `<link>` no `<head>` do documento —
é assim que `assets/tailwind.css` (gerado pelo Tailwind a partir de `tailwind.css`) é
carregado.

## Passeio guiado pelo código, arquivo por arquivo

### `src/main.rs`

Ponto de entrada do binário. `fn main()` chama `dioxus::launch(App)`; `App` provê o
contexto de acessibilidade (`A11ySettings::provide()`), aplica o zoom de fonte
(`use_apply_font_scale`), injeta favicon/CSS e renderiza `Router::<Route> {}` — a partir
daqui, quem decide o que aparece na tela é a rota atual (ver `routes.rs`).

### `src/routes.rs`

Define o `enum Route` (as três rotas do app) e o componente `AppLayout`, compartilhado
por todas elas. `AppLayout` lê `A11ySettings` do contexto e monta a `class` do `div` raiz
com base em `high_contrast`.

### `src/accessibility.rs`

`A11ySettings` guarda dois `Signal`s (`font_scale: Signal<i32>`,
`high_contrast: Signal<bool>`) e métodos (`increase_font`,
`decrease_font`, `reset_font`, `toggle_contrast`) que leem o valor
atual e escrevem o novo, sempre "clampando" (`.min(2)`/`.max(-1)`) para o zoom de fonte
não passar de um limite. `use_apply_font_scale` e `speak` são as duas pontes com
JavaScript do navegador (ver seção "`use_effect`" acima).

### `src/data.rs`

Todos os tipos de dado do app (`Service`, `Category`, `Alert`/`AlertLevel`,
`LifeArea`/`LifeState`, `AttendancePoint`, `HelpReason`, `FraudRiskLevel`, `FraudSignal`)
e as funções que devolvem listas fixas desses tipos. É o arquivo mais "burocrático" do
projeto — nenhuma lógica de UI, só modelagem de dados — e é a **fonte única de
verdade**: nenhum componente duplica um array próprio de serviços/categorias, todos
chamam `data::services()`/`data::categories()`/etc.

### `src/components/header.rs`

Cabeçalho fixo (`sticky top-0`) em duas partes: uma barra fina com links de pulo ("Ir
para o conteúdo" etc., escondidos visualmente com `sr-only` mas acessíveis a leitores de
tela) e os controles de acessibilidade (A-/A/A+/alto contraste); depois o
cabeçalho principal com logo, menu (`nav`) e o botão "Entrar com gov.br". O menu mobile é
um segundo `nav` que só aparece quando `menu_open()` é `true`.

### `src/components/hero.rs`

Título simples + barra de busca com autocomplete real (`use_memo` filtrando
`data::services()` por nome/tag, ver seção "Aula de Dioxus"). Cada resultado é um `Link`
para `Route::ServiceDetail`.

### `src/components/assistant.rs`

Campo de texto livre + reconhecimento por palavras-chave (sem IA/backend), explicado em
detalhe na seção "Iteradores e closures" acima. Também tem botões de exemplo prontos
(`examples`) que preenchem a pergunta com um clique.

### `src/components/stats.rs`, `security.rs`, `status.rs`, `categories.rs`

Seções "vitrine" da home, todas com o mesmo formato: um array local (`struct Feature`/
`Indicator`/`Stat`, ou `data::categories()`) percorrido com `for` dentro de um grid
Tailwind (`grid sm:grid-cols-2 lg:grid-cols-4 gap-4`, etc.). São os componentes mais
simples do projeto — bom ponto de partida para quem está aprendendo `rsx!`.

### `src/components/services.rs`

Lista numerada "Serviços para você", usando `.enumerate()` para mostrar `{i + 1}` ao lado
de cada serviço (imita a lista numerada do gov.br real, em vez de cards com borda).

### `src/components/performance.rs`

`PerformanceCard(service: Service)` — recebe um `Service` inteiro como prop e mostra
nota (estrelas geradas com `for i in 0..5 { span { if i < full_stars {"★"} else {"☆"} } }`),
tempo médio, percentual dentro do prazo e principal reclamação. Renderizado dentro de
`ServiceDetail`.

### `src/components/fraud_shield.rs`

`FraudShield` — card de transparência do "sistema antifraude" simulado: pega o score
fixo (`fraud_risk_score()`, hoje sempre `34`) e a faixa (`fraud_level`) de `data.rs`,
escolhe cor/rótulo com um `match` sobre `FraudRiskLevel`, desenha uma barra de progresso
e lista os 7 `fraud_signals()` (cada um com um ✓ ou ⚠ conforme o campo `ok: bool`). A
ideia é mostrar que uma decisão de risco combina **vários sinais independentes**, não uma
regra única.

### `src/components/alerts.rs`

`AlertsBell` — o sino 🔔 no cabeçalho. Um `Signal<bool>` (`open`) controla um dropdown
que lista `data::alerts()`; a cor da bolinha ao lado de cada alerta vem de um `match` em
`AlertLevel` (ver seção "Enums e match" acima).

### `src/components/help_widget.rs`

`HelpWidget` — botão flutuante fixo ("Não consigo resolver"), presente em toda página
via `AppLayout`. Abre um modal com dois estados controlados por signals: lista de
motivos (`help_reasons()`) ou, depois de escolher um
(`selected: Signal<Option<usize>>`), a solução sugerida + postos de atendimento mais
próximos (`attendance_points()`).

### `src/components/feedback.rs`

`FeedbackForm` — avaliação por estrelas (1 a 5) + checklist de problemas opcionais
(`checked: Signal<Vec<bool>>`, um `bool` por item de `ISSUES`). O botão "Enviar" fica
desabilitado (`disabled: rating() == 0`) até uma nota ser escolhida.

### `src/components/footer.rs`

Rodapé institucional (cinco colunas de links) sem nenhum estado — só `rsx!` estático.
Contém o aviso obrigatório *"não é um site oficial do governo"*.

### `src/pages/home.rs`

Junta os componentes da home na ordem em que aparecem na tela (`Hero`, `Assistant`,
`Stats`, `Services`, `Categories`, `Security`, `StatusPanel`) e registra, via
`use_effect` + `document::eval`, o `IntersectionObserver` que anima a entrada de cada
seção marcada com a classe `.reveal` (ver seção "`use_effect`" acima).

### `src/pages/service_detail.rs`

A página mais complexa do projeto: uma pequena **máquina de estados** para simular o
fluxo de contratar um serviço, controlada por vários signals (`confirmed`, `started`,
`face_check: Signal<FaceCheck>`, `step: Signal<usize>`, `simple_lang`). A ordem das telas
é decidida por uma cadeia de `if`/`else if`/`else` no `rsx!`:

1. Busca o serviço pelo slug (`Option<Service>`) — se não achar, mostra "Serviço não
   encontrado".
2. **Antes de começar**: checklist de requisitos + checkbox de confirmação
   (`s.requirements.iter().copied()`).
3. Se o serviço exige biometria (`s.needs_biometrics`) e ainda não foi resolvida: simula
   uma verificação facial que "falha" (`FaceCheck::Failed`) e oferece alternativas
   (banco credenciado, e-mail, telefone, presencial) até ficar `FaceCheck::Resolved`.
4. Caso contrário: um stepper visual de 6 etapas (`PROCESS_STEPS`), avançado manualmente
   pelo botão "Simular avanço da etapa"; ao concluir, mostra `FeedbackForm`.

Também tem o botão "🧠 Linguagem simples" (alterna entre `s.description` e
`s.simple_explanation`) e "🔊 Ouvir" (chama `speak()` com o texto atual em tela).

### `src/pages/my_gov.rs`

`MyGovPanel` — o painel "Minha vida no governo": lista `data::alerts()` no topo, o
`FraudShield`, e depois um grid de `life_areas()` (cada `LifeArea` colore o texto de
status conforme `LifeState` com um `match`, e vira um `Link` para o serviço relacionado
quando `related_slug` é `Some(slug)`).

## Glossário rápido

| Termo | O que é | Onde aparece |
|---|---|---|
| `struct` | Tipo que agrupa campos nomeados | `Service`, `Category` em [src/data.rs](src/data.rs) |
| `enum` | Tipo com um conjunto fixo de variantes | `AlertLevel`, `LifeState`, `FraudRiskLevel` |
| `match` | Escolhe um braço de código pela variante/valor; o compilador exige cobrir todos os casos | `fraud_level()`, `AppLayout`, `alerts.rs` |
| `Option<T>` | `Some(valor)` ou `None` — ausência de valor sem `null` | `service: Option<Service>`, `related_slug` |
| `derive` | Gera automaticamente a implementação de uma trait | `#[derive(Clone, PartialEq)]` |
| `Clone` / `Copy` | Traits que permitem duplicar um valor explicitamente (`Clone`) ou implicitamente (`Copy`, só tipos "baratos") | `Service: Clone`, `A11ySettings: Clone, Copy` |
| `&'static str` | Fatia de texto fixa, vive pelo programa inteiro, sem alocação | Campos de texto em `data.rs` |
| `Vec<T>` | Lista de tamanho dinâmico alocada no heap | Retorno de `services()`, `categories()` |
| closure | Função anônima que captura variáveis do escopo (`\|x\|` ou `move \|_\|`) | `.filter(\|s\| ...)`, `onclick: move \|_\| ...` |
| `#[component]` | Marca uma função Rust como componente Dioxus | Toda função em `components/` e `pages/` |
| `Element` | Tipo de retorno de um componente Dioxus | Toda função `#[component]` |
| `rsx!` | Macro que descreve a árvore de UI | Corpo de todo componente |
| `Signal<T>` | Container de estado reativo; ler dispara re-render quando muda | `use_signal`, `A11ySettings` |
| `use_signal` | Hook que cria um `Signal` local ao componente | `use_signal(\|\| false)` |
| `use_memo` | Valor derivado, recalculado só quando dependências mudam | `results`/`matches` em `hero.rs`/`assistant.rs` |
| `use_effect` | Roda uma closure após o render, de novo quando signals lidos mudam | Zoom de fonte, `IntersectionObserver` |
| `use_context_provider` / `use_context` | Compartilha estado sem passar por props em cada nível | `A11ySettings` |
| `Routable` / `#[route]` | Derive + atributo que transformam um `enum` em roteador | `enum Route` em `routes.rs` |
| `#[layout(...)]` / `Outlet` | Layout compartilhado entre rotas + ponto de inserção da página atual | `AppLayout`, `Outlet::<Route> {}` |
| `Link` | Navegação tipada, verificada em tempo de compilação | `Link { to: Route::ServiceDetail {...} }` |
| `asset!` | Resolve um caminho de arquivo em tempo de compilação | `asset!("/assets/tailwind.css")` |
| `document::eval` | Executa uma string de JavaScript no navegador | Zoom de fonte, fala, `IntersectionObserver` |

## Problema conhecido nesta máquina: Smart App Control

Se `dx serve` ou `cargo build` falharem com `Uma política de Controle de Aplicativo
bloqueou este arquivo` (os error 4551), o **Controle de Aplicativos Inteligente** do
Windows está bloqueando a execução dos build scripts do Cargo. Desative em
Configurações → Privacidade e segurança → Segurança do Windows → Controle de
aplicativos e do navegador → Controle de Aplicativos Inteligente. Atenção: uma vez
desativado, só é possível reativar reinstalando o Windows.

## Para continuar aprendendo

- [AGENTS.md](AGENTS.md) — referência rápida da API do Dioxus 0.7.
- [SETUP.md](SETUP.md) — como montar este ambiente do zero, passo a passo.
- [Documentação oficial do Dioxus 0.7](https://dioxuslabs.com/learn/0.7/) — livro
  completo, com mais exemplos de signals, roteamento, fullstack e async.
- [The Rust Book](https://doc.rust-lang.org/book/) — para revisar com calma os
  fundamentos da linguagem (ownership, traits, enums, etc.).

## Licença

Distribuído sob a licença MIT — veja [LICENSE](LICENSE).

