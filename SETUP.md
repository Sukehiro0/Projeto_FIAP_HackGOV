# Guia: Como montar um ambiente de front-end Rust (Dioxus + Tailwind CSS) no Windows

Este guia reproduz, passo a passo, tudo que foi feito para preparar este projeto do
zero em uma máquina Windows limpa. Todos os comandos são para **PowerShell**.

## 0. Pré-requisitos

- Windows 10/11 com [winget](https://learn.microsoft.com/pt-br/windows/package-manager/winget/)
  (já vem instalado em versões recentes do Windows)
- [Node.js](https://nodejs.org/) instalado (usado só para compilar o Tailwind CSS)
- Conexão com a internet

Verifique se já tem Node/npm instalados:

```powershell
node --version
npm --version
```

Se não tiver, instale antes de continuar: `winget install --id OpenJS.NodeJS -e --source winget`.

---

## 1. Instalar o Rust (rustup)

```powershell
winget install --id Rustlang.Rustup -e --source winget --silent --accept-package-agreements --accept-source-agreements
```

> Se aparecer um erro `Failed when searching source: msstore` / erro de certificado,
> é só isso mesmo — o `--source winget` no comando acima já contorna o problema.

Depois de instalar, **abra um novo terminal** (para o PATH ser atualizado) ou rode
nesta mesma sessão:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
```

Confirme a instalação:

```powershell
rustc --version
cargo --version
```

## 2. Adicionar o target WebAssembly

Front-end em Rust compila para WebAssembly (WASM), então precisamos do target certo:

```powershell
rustup target add wasm32-unknown-unknown
```

## 3. Instalar o linker (Visual Studio Build Tools)

No Windows, o Rust precisa do linker do MSVC para compilar qualquer coisa (não só
Dioxus). Sem isso, a compilação falha com `error: linker link.exe not found`.

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --source winget --silent --accept-package-agreements --accept-source-agreements --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

> Este passo demora bastante (baixa alguns GB). É normal a instalação ficar "parada"
> por vários minutos antes de terminar.

## 4. Instalar a Dioxus CLI (`dx`)

```powershell
cargo install dioxus-cli --locked
```

> Esse comando compila ~800 crates na primeira vez — também demora bastante (pode
> levar mais de 10 minutos dependendo da máquina). Ao final, confirme que funcionou:
> ```powershell
> dx --version
> ```

## 5. Criar o projeto Dioxus

```powershell
dx new nome-do-projeto
```

O comando é **interativo** e pergunta, em ordem:

1. **Which sub-template should be expanded?** → escolha `Bare-Bones` (mínimo) ou
   `Jumpstart` (mais completo, com exemplos)
2. **Do you want to use Dioxus Fullstack?** → `false` (front-end puro, sem servidor)
3. **Do you want to use Dioxus Router?** → `true` se for ter múltiplas páginas/rotas
4. **Do you want to use Tailwind CSS?** → `true`
5. **Do you want to include prompts for LLMs?** → opcional (gera um `AGENTS.md` com
   referência da API do Dioxus, útil para o GitHub Copilot)
6. **Which platform do you want DX to serve by default?** → `Web`

> ⚠️ **Atenção**: esse menu se navega com as **setas do teclado (↑/↓) + Enter**, direto
> no terminal. Se você copiar/colar comandos automatizados ou usar um terminal que não
> repassa teclas de seta corretamente, ele pode simplesmente confirmar a opção padrão
> (`false`) sem mudar nada — preste atenção no que fica destacado (`❯`) antes de
> apertar Enter. Se errar alguma resposta, não tem problema: dá pra configurar tudo
> manualmente depois (Tailwind e Router incluídos), como mostram os passos seguintes.

Depois de criado, entre na pasta:

```powershell
cd nome-do-projeto
```

## 6. Configurar o Tailwind CSS manualmente (se necessário)

Se o passo 5 já criou o projeto com Tailwind funcionando (arquivo `tailwind.css` +
referência em `src/main.rs`), pule esta seção. Caso contrário, configure assim:

```powershell
npm init -y
npm install -D tailwindcss @tailwindcss/cli
```

Crie um arquivo `tailwind.css` na raiz do projeto com o conteúdo:

```css
@import "tailwindcss";
```

Adicione estes scripts no `package.json` gerado:

```json
{
  "scripts": {
    "watch:css": "tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch",
    "build:css": "tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --minify"
  }
}
```

Gere o primeiro build do CSS:

```powershell
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
```

Edite `src/main.rs` para carregar o CSS gerado:

```rust
use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Home {}
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "min-h-screen bg-slate-900 text-white flex flex-col items-center justify-center gap-6 p-8",
            h1 { class: "text-4xl font-bold", "🦀 Meu App" }
            div { class: "flex items-center gap-4",
                button {
                    class: "w-10 h-10 rounded bg-slate-700 hover:bg-slate-600",
                    onclick: move |_| count -= 1,
                    "-"
                }
                span { class: "text-2xl font-mono w-12 text-center", "{count}" }
                button {
                    class: "w-10 h-10 rounded bg-indigo-600 hover:bg-indigo-500",
                    onclick: move |_| count += 1,
                    "+"
                }
            }
        }
    }
}
```

## 7. Configurar o VS Code

Instale as extensões (Ctrl+Shift+X e busque, ou pelo terminal com o VS Code no PATH):

```powershell
code --install-extension rust-lang.rust-analyzer
code --install-extension bradlc.vscode-tailwindcss
code --install-extension tamasfe.even-better-toml
```

Crie o arquivo `.vscode/settings.json` para o autocomplete do Tailwind funcionar
dentro das strings `class: "..."` do código Rust (por padrão ele só funciona em
HTML/JSX):

```json
{
  "tailwindCSS.includeLanguages": {
    "rust": "html"
  },
  "tailwindCSS.experimental.classRegex": [
    ["class:\\s*\"([^\"]*)\"", "([a-zA-Z0-9\\-:/%.\\[\\]]+)"]
  ],
  "rust-analyzer.cargo.features": ["web"],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

## 8. Rodar o projeto

Em um terminal, deixe o Tailwind observando mudanças de classes:

```powershell
npm run watch:css
```

Em outro terminal, sirva o app com hot-reload:

```powershell
dx serve
```

Acesse a URL mostrada no terminal (normalmente `http://127.0.0.1:8080`).

---

## Problemas comuns

### `error: linker link.exe not found`

Faltou o passo 3 (Visual Studio Build Tools). Instale e tente de novo.

### `Uma política de Controle de Aplicativo bloqueou este arquivo` (os error 4551)

O **Controle de Aplicativos Inteligente (Smart App Control)** do Windows bloqueia
binários não assinados — incluindo os que o Cargo compila e executa durante o build
(build scripts). Isso trava **qualquer** build Rust na máquina, não só este projeto.

Para verificar se está ativado:

```powershell
Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\CI\Policy" -Name "VerifiedAndReputablePolicyState" -ErrorAction SilentlyContinue
```

- `1` = ativado (bloqueia builds Rust)
- `2` = modo avaliação
- `0` ou erro = desativado

Para desativar: **Configurações → Privacidade e segurança → Segurança do Windows →
Controle de aplicativos e do navegador → Controle de Aplicativos Inteligente → Desligar**.

> ⚠️ Uma vez desativado, só é possível reativar reinstalando o Windows do zero. É uma
> decisão que cada pessoa deve tomar conscientemente antes de desenvolver em Rust
> nesta máquina.

### `Failed when searching source: msstore` no winget

Erro de certificado de uma fonte específica do winget nesta máquina. Sempre use
`--source winget` explicitamente nos comandos de instalação (como já está nos
comandos deste guia).
