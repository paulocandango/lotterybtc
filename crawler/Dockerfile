# Etapa de build
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /app

# Copia os arquivos de configuração e dependências do `crawler`
COPY ./crawler/Cargo.toml ./crawler/Cargo.lock ./

# Copia o código-fonte do `crawler`
COPY ./crawler ./crawler

# Copia a pasta `shared`
COPY ./shared ./shared

# Constrói o projeto `crawler`
RUN cargo build --release --manifest-path ./crawler/Cargo.toml

# Etapa final: reduz a imagem para produção
FROM debian:bookworm-slim

# Define o diretório de trabalho
WORKDIR /app

# Copia o binário compilado do `crawler`
COPY --from=builder /app/crawler/target/release/crawler /app/crawler

# Define o ponto de entrada para o contêiner
ENTRYPOINT ["/app/crawler"]