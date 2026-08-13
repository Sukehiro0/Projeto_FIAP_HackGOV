# HackGOV — Front-end em Rust (Dioxus + Tailwind CSS)

Projeto front-end escrito em Rust, compilado para WebAssembly, usando o framework
[Dioxus](https://dioxuslabs.com/) (v0.7) com [Tailwind CSS](https://tailwindcss.com/) (v4)
para estilização.

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

## Estrutura do projeto

```
├─ assets/            # Imagens, ícones e o CSS compilado (tailwind.css)
├─ src/
│  └─ main.rs         # Ponto de entrada e componentes da aplicação
├─ tailwind.css        # Entrada do Tailwind (@import "tailwindcss";)
├─ Cargo.toml          # Dependências e features do Rust
├─ Dioxus.toml         # Configuração do Dioxus CLI (título, assets, etc.)
├─ package.json        # Apenas as dependências de build do Tailwind CSS
└─ AGENTS.md           # Referência rápida da API do Dioxus 0.7 (útil para o Copilot)
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

## Problema conhecido nesta máquina: Smart App Control

Se `dx serve` ou `cargo build` falharem com `Uma política de Controle de Aplicativo
bloqueou este arquivo` (os error 4551), o **Controle de Aplicativos Inteligente** do
Windows está bloqueando a execução dos build scripts do Cargo. Desative em
Configurações → Privacidade e segurança → Segurança do Windows → Controle de
aplicativos e do navegador → Controle de Aplicativos Inteligente. Atenção: uma vez
desativado, só é possível reativar reinstalando o Windows.

