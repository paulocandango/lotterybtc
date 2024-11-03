# Etapa de build
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /app

# Copia os arquivos de configuração e dependências do `crawler`
COPY ./Cargo.toml ./Cargo.lock ./

# Copia o código-fonte do `crawler`
COPY . .

# Constrói o projeto `crawler`
RUN cargo build --release --manifest-path ./Cargo.toml

# Etapa final: reduz a imagem para produção
FROM alpine:latest
RUN apk add --no-cache libc6-compat gcc

# Define o diretório de trabalho
WORKDIR /app

# Copia o binário compilado do `crawler`
COPY --from=builder /app/target/release/lotterybtc-server /app/lotterybtc-server

# Define o ponto de entrada para o contêiner
ENTRYPOINT ["/app/lotterybtc-server"]