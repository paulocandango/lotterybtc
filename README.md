# Projeto LotteryBTC

LotteryBTC é um software projetado para capturar e exibir resultados de sorteios de loterias de 
diferentes fontes da web, utilizando Rust para alto desempenho e confiabilidade. O projeto também 
permite que usuários façam apostas personalizadas, utilizando um sistema de banco de dados e uma 
interface web intuitiva. Este projeto foi dividido em duas partes principais: um **web crawler** 
e um **servidor web**, além de incluir uma aplicação Android para facilitar o acesso aos dados e 
funcionalidades.

## Estrutura do Projeto

O projeto está organizado em duas partes principais e uma aplicação móvel:

1. **Web Crawler**: 
   - **Objetivo**: Executado periodicamente para capturar dados de sorteios de loterias, 
     armazenando essas informações em um banco de dados PostgreSQL.
   - **Tecnologias**: Rust, Podman, Thirtyfour (para web scraping), AWS Fargate.
   - **Descrição**: Esse crawler abre navegadores automatizados para visitar sites específicos, 
     identifica os números sorteados e salva os resultados no banco de dados para posterior consulta.

2. **Servidor Web**:
   - **Objetivo**: Fornece uma interface web para exibir os resultados dos sorteios e permite que os 
     usuários façam apostas, escolhendo combinações específicas.
   - **Tecnologias**: Rust (Actix Web para o backend), Tera para templates, PostgreSQL.
   - **Descrição**: O servidor é configurado para rodar continuamente, mas devido ao baixo tráfego 
     inicial, pode ser configurado para iniciar sob demanda. Ele exibe os dados capturados e permite 
     que os usuários interajam com o sistema.

3. **Aplicação Android**:
   - **Objetivo**: Oferecer uma interface móvel para acessar dados e realizar apostas.
   - **Tecnologias**: Android Studio, Kotlin.
   - **Descrição**: Este aplicativo permite aos usuários consultar rapidamente os resultados de 
     sorteios e registrar apostas usando um dispositivo Android.

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação principal, escolhida pela eficiência e segurança.
- **Thirtyfour**: Biblioteca para automação de navegação web (web scraping).
- **Podman**: Utilizado para criar contêineres independentes e facilitar a implantação.
- **AWS Fargate**: Infraestrutura de execução para o web crawler, escalando conforme necessário.
- **Actix Web**: Framework web para o servidor backend.
- **PostgreSQL**: Banco de dados para armazenamento de resultados e apostas.
- **Android Studio**: IDE para o desenvolvimento do aplicativo Android.

## Estrutura de Diretórios

A organização dos diretórios principais do projeto:

lotterybtc/ 
├── docs/ # Documentação do projeto 
├── src/ 
│ ├── crawler/ # Código do web crawler 
│ ├── server/ # Código do servidor web 
│ └── drivers/ # Drivers de navegadores (ex.: GeckoDriver, ChromeDriver) 
├── scripts/ # Scripts auxiliares 
├── config/ # Arquivos de configuração (.env, YAML) 
├── logs/ # Logs do sistema 
├── bin/ # Binários finais gerados 
├── infrastructure/ # Arquivos de configuração do Podman e Docker 
├── tests/ # Scripts de teste 
├── install/ # Instaladores (não versionados no Git) 
└── android_app/ # Código fonte do aplicativo Android

## Funcionamento do Projeto

1. **Execução do Web Crawler**:
   - A cada hora, a imagem em contêiner do crawler é inicializada na AWS Fargate.
   - O crawler utiliza Thirtyfour para abrir um navegador, visitar os sites de loteria, e coletar 
     informações de sorteios.
   - Os dados capturados são então salvos no banco de dados PostgreSQL.

2. **Servidor Web**:
   - O servidor exibe os dados capturados em uma página web.
   - Usuários podem visualizar os resultados dos sorteios e registrar apostas personalizadas.

3. **Aplicativo Android**:
   - Permite que os usuários consultem os resultados de sorteios e realizem apostas por meio de uma 
     interface móvel amigável.

## Como Configurar o Projeto

1. **Pré-requisitos**:
   - Rust instalado (https://www.rust-lang.org/tools/install)
   - Podman instalado (https://podman.io/getting-started/installation)
   - PostgreSQL configurado e em execução
   - Android Studio para desenvolvimento do aplicativo móvel

2. **Configuração do Banco de Dados**:
   - Configure o banco de dados PostgreSQL e adicione as tabelas necessárias para armazenar os 
     resultados e as apostas dos usuários.

3. **Build e Execução**:
   - **Web Crawler**: Construa e implemente o contêiner do crawler usando Podman e AWS Fargate.
   - **Servidor Web**: Construa o binário e inicie o servidor Actix Web.
   - **Aplicativo Android**: Compile o código Android no Android Studio e instale o APK no dispositivo.

## Contribuição

Este projeto é aberto a contribuições. Para contribuir, faça um fork deste repositório, crie uma 
nova branch para suas alterações e submeta um pull request detalhando as modificações.

---

Esse README fornece uma visão geral do funcionamento, organização e implantação do LotteryBTC. 
Para mais informações, consulte a documentação completa no diretório `docs/`.