# Etapa de build
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /app

# Copia os arquivos de configuração e dependências do `crawler`
COPY ./Cargo.toml ./Cargo.lock ./
COPY . .

# Copia o arquivo de ambiente
COPY .env.prod .env

# Constrói o projeto `crawler`
RUN cargo build --release --manifest-path ./Cargo.toml

# Etapa final: reduz a imagem para produção
FROM debian:stable-slim

# Define o frontend como não interativo
ENV DEBIAN_FRONTEND=noninteractive

# Instala as dependências essenciais, incluindo Chromium e ChromeDriver
RUN apt-get update && \
    apt-get install -y libc6 wget unzip chromium chromium-driver && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Variáveis de ambiente para Chromium
ENV CHROME_BIN=/usr/bin/chromium
ENV CHROME_PATH=/usr/lib/chromium/

# Define o diretório de trabalho
WORKDIR /app

# Copia o binário compilado do `crawler`
COPY --from=builder /app/target/release/lotterybtc-crawler /app/lotterybtc-crawler

# Define o ponto de entrada para o contêiner
ENTRYPOINT ["/app/lotterybtc-crawler"]
