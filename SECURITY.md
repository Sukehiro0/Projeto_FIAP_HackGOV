# Segurança — hack.gov.br (HackGOV)

Este documento existe para definir e seguir as práticas da 
OWASP e as defesas relevantes contra ataques recentes a sites de governo.
Ele está dividido em duas partes:

1. **O que já foi implementado** neste front-end.
2. **O que fica documentado, mas não implementado**, porque depende de
   coisas que um front-end estático sozinho não controla (servidor, CDN,
   backend, processo de identidade real) — a ideia é que isso sirva de
   checklist para quando o projeto ganhar um backend de verdade.

Importante: **este é um front-end estático (SPA em WASM), sem backend
próprio.** Isso muda bastante o que é "aplicável": itens do OWASP Top 10
como injeção de SQL, quebra de autenticação no servidor, IDOR, etc. não têm
onde acontecer hoje, porque não existe servidor de aplicação nem banco de
dados neste repositório. Quando o projeto ganhar um backend (mesmo que
mock), este documento precisa ser revisitado.

---

## 1. Implementado

### CSP (Content Security Policy) — `src/main.rs`
Meta tag `Content-Security-Policy` restritiva: `default-src 'self'`, sem
scripts/estilos/fontes/imagens de terceiros, `object-src 'none'`,
`base-uri 'self'`, `form-action 'self'`. Isso mitiga a maior parte dos
cenários de XSS refletido/DOM-based e de exfiltração de dados via tags
injetadas, mesmo que uma vulnerabilidade de injeção apareça no futuro.

Duas concessões conscientes, documentadas em comentário no próprio código:
- `'unsafe-inline'` em `style-src`: usado pela barra de risco do
  `FraudShield`, que precisa de largura dinâmica via atributo `style`.
- `'unsafe-eval'` / `'wasm-unsafe-eval'` em `script-src`: exigido pelo
  mecanismo `document::eval` do Dioxus, usado em dois pontos (leitura em
  voz alta e o observer de scroll da Home) para chamar APIs do navegador
  que o framework ainda não expõe como hook nativo.

### Zero dependências externas de UI
Trocamos todos os ícones de emoji por SVG desenhado localmente
(`src/components/icons.rs`), compilado dentro do `.wasm`. Isso não é só
estético: elimina qualquer origem de terceiros para ícones (nada de CDN de
ícone, nada de fonte de terceiro), reduzindo a superfície de ataque de
cadeia de suprimentos e simplificando o CSP (não precisamos liberar nenhum
domínio extra em `script-src`/`font-src`/`img-src`).

### Sanitização defensiva no `document::eval`
`accessibility.rs::speak()` escapa aspas, barras invertidas, quebras de
linha e os separadores de linha Unicode U+2028/U+2029 antes de interpolar
texto dentro de uma string JavaScript. Hoje esse texto só vem de conteúdo
estático (`data.rs`), então o risco real é baixo — mas o código já está
preparado para não quebrar/injetar caso esse texto passe a vir de uma fonte
dinâmica no futuro (comentário no código aponta a migração recomendada:
trocar `format!` por passagem de valor tipada).

### Referrer-Policy
`<meta name="referrer" content="strict-origin-when-cross-origin">` evita
vazar a URL completa (que pode conter, futuramente, tokens ou parâmetros
sensíveis) para sites de terceiros ao clicar em links externos.

### Nenhum dado sensível em `localStorage`/`sessionStorage`
Conferido: a aplicação não persiste nada no navegador hoje. Se/quando isso
mudar (ex: lembrar preferências de acessibilidade entre sessões), a
recomendação é usar apenas dados não sensíveis (não guardar nada que
pareça identidade, documento, ou sessão).

---

## 2. Documentado, não implementável só no front-end

Estes itens exigem infraestrutura (servidor, CDN, WAF) ou um backend real,
que não existe neste repositório. Ficam aqui como checklist para quando
existirem.

| Item | Por que não dá pra fazer só no front-end | O que fazer quando houver backend/infra |
|---|---|---|
| `X-Frame-Options` / `frame-ancestors` real | O navegador **ignora** `frame-ancestors` quando entregue via `<meta>` — só funciona como cabeçalho HTTP de verdade. | Configurar no servidor/CDN: `X-Frame-Options: DENY` + `Content-Security-Policy: frame-ancestors 'self'` como header HTTP. |
| `Strict-Transport-Security` (HSTS) | É um cabeçalho HTTP enviado pelo servidor na resposta HTTPS; não existe equivalente em `<meta>`. | `Strict-Transport-Security: max-age=63072000; includeSubDomains; preload`. |
| `X-Content-Type-Options: nosniff` | Mesma limitação — cabeçalho HTTP only. | Configurar no servidor/CDN. |
| Rate limiting / anti-automação | Não tem como limitar taxa de requisições sem um servidor no meio (o SPA roda inteiro no navegador do usuário). | WAF ou API gateway na frente do backend real, com rate limiting por IP/sessão e captcha/desafio em endpoints sensíveis (login, recuperação de senha). |
| Autenticação e sessão reais | O "Entrar com gov.br" e a verificação facial hoje são simulações client-side, sem qualquer emissão de token. | Implementar OAuth2/OIDC de verdade (ex: integração real com o provedor de identidade gov.br), com tokens de sessão `HttpOnly` + `Secure` + `SameSite=Strict`, nunca em `localStorage`. |
| Proteção contra phishing/typosquatting de domínios parecidos | Ataques recentes a portais de governo brasileiros usam domínios muito parecidos (ex: trocar `.gov.br` por `.com`) para roubar credenciais — isso se combate no nível de DNS/registro de domínio e campanhas de conscientização, não no código do site. | Registrar/monitorar variações do domínio; usar `DMARC`/`SPF`/`DKIM` nos e-mails oficiais; considerar Certificate Transparency monitoring para detectar certificados emitidos indevidamente para domínios parecidos. |
| Auditoria de dependências (`cargo audit` / `npm audit`) | É um processo de CI, não uma linha de código da aplicação. | Adicionar `cargo audit` e `npm audit` ao pipeline de CI, rodando a cada PR. |
| WAF / proteção contra DDoS | Exige uma camada de rede na frente da aplicação. | Cloudflare, AWS WAF, ou equivalente, na frente do domínio real. |
| Registro e monitoramento de acesso (logging/SIEM) | Sem backend, não existe onde logar tentativas de acesso. | Backend real deve logar tentativas de autenticação, erros 4xx/5xx repetidos, e alimentar um SIEM. |
| Verificação facial e biometria reais | O simulador de reconhecimento facial deste projeto é só uma tela de demonstração, sem processamento real de imagem nem envio de dados biométricos. | Se a intenção é ter isso pra valer futuramente, isso é dado sensível sob a LGPD (dado biométrico) e exige DPIA (avaliação de impacto), consentimento explícito e um fornecedor certificado — não é algo pra "implementar rápido" num front-end. |

---

## 3. Backlog técnico relacionado (não é bug de segurança, mas afeta a robustez)

- Migrar as duas chamadas de `document::eval` para bindings `wasm-bindgen`
  tipados, para poder remover `'unsafe-eval'` do CSP.
- Reescrever a barra de risco do `FraudShield` sem atributo `style` inline
  (ex: um pequeno componente que escolhe entre um conjunto fixo de larguras
  pré-compiladas pelo Tailwind), para poder remover `'unsafe-inline'` do
  CSP de `style-src`.
- Definir `lang="pt-BR"` no elemento `<html>` raiz (bom para acessibilidade
  e para leitores de tela). Isso exige um `index.html` customizado; não foi
  feito nesta rodada para não arriscar quebrar o build sem conseguir testar
  localmente — ver `dx serve` antes de aplicar.
