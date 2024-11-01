# Etapa de build
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /app

# Copia os arquivos de configuração e dependências do `crawler`
COPY ./server/Cargo.toml ./server/Cargo.lock ./

# Copia o código-fonte do `crawler`
COPY ./server ./server

# Copia a pasta `shared`
COPY ./shared ./shared

# Constrói o projeto `crawler`
RUN cargo build --release --manifest-path ./server/Cargo.toml

# Etapa final: reduz a imagem para produção
FROM debian:bookworm-slim

# Define o diretório de trabalho
WORKDIR /app

# Copia o binário compilado do `crawler`
COPY --from=builder /app/server/target/release/server /app/server

# Define o ponto de entrada para o contêiner
ENTRYPOINT ["/app/server"]