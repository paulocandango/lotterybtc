cd C:\dev\lotterybtc
podman build -t crawler-prod -f crawler/Dockerfile .
podman run crawler-prod

Exportar e Importar e Executar
podman save -o crawler-prod.tar localhost/crawler-prod:latest
podman load -i crawler-prod.tar
podman run -it --rm crawler-prod