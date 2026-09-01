# syntax=docker/dockerfile:1
#
# Build multi-estágio (cargo-chef) para o backend fullstack do HackGOV,
# seguindo o guia oficial de deploy do Dioxus 0.7 para Fly.io:
# https://dioxuslabs.com/learn/0.7/tutorial/deploy
#
# Uso local:
#   docker build -t hackgov .
#   docker run -p 8080:8080 -v hackgov_data:/data hackgov

FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Node.js só é necessário para compilar o Tailwind CSS antes do bundle do Dioxus
# (a versão via apt do Debian costuma ser antiga demais para o Tailwind CSS v4).
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
RUN npm ci && npm run build:css

# Instala o Dioxus CLI (dx) via cargo-binstall, na mesma versão usada no CI/Pages.
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --version 0.7.10 --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Gera o bundle final: cliente web (WASM) + binário nativo do servidor (feature `server`).
RUN dx bundle --web --release

FROM chef AS runtime
COPY --from=builder /app/target/dx/hackgov/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0
# Caminho do banco SQLite — aponte para um volume persistente em produção (ver fly.toml).
ENV HACKGOV_DB_PATH=/data/hackgov.db

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
