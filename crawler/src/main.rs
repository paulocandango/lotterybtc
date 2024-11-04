use dotenv::dotenv;
use tokio;
use std::time::Duration;
use tokio::time::sleep;
use thirtyfour::prelude::*;
use std::process::Command;
use thirtyfour::ChromeCapabilities;

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    println!("INICIANDO LotteryBtc-Crawler!");

    dotenv().ok();
    let chromedriver_path = dotenv::var("CHROMEDRIVER_PATH").unwrap_or_else(|_| "chromedriver".to_string());
    println!("CHROMEDRIVER_PATH: {}", chromedriver_path);

    // Inicia o ChromeDriver
    let chromedriver = Command::new(chromedriver_path)
        .arg("--port=9515")
        .spawn()
        .expect("Falha ao iniciar o ChromeDriver");

    // Aguarda um tempo para garantir que o ChromeDriver esteja pronto
    sleep(Duration::from_secs(2)).await;

    // Configura o WebDriver para o Chrome headless
    let mut caps = ChromeCapabilities::new();
    caps.set_headless()?;
    caps.set_disable_gpu()?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--remote-debugging-port=9222")?;
    caps.add_arg("--window-size=1920,1080")?;
    caps.add_arg("--disable-blink-features=AutomationControlled")?;
    caps.add_arg("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.54 Safari/537.36")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    sleep(Duration::from_secs(2)).await;


    let url = "http://loterias.caixa.gov.br/Paginas/Mega-Sena.aspx";
    let page_source = driver.get(url).await?;
    println!("Página {} carregada com sucesso!", url);
    println!("CONTEUDO COMPLETO: {:?}", page_source);

    // Define o tempo máximo de espera
    let css = "ul#ulDezenas li";
    let timeout = Duration::from_secs(20);
    let mut elapsed = Duration::from_secs(0);
    let interval = Duration::from_millis(3000); // Intervalo entre cada verificação

    // Loop para aguardar até que 6 elementos sejam encontrados ou o tempo máximo seja atingido
    let mut elements;
    loop {

        let page_source = driver.source().await?;
        println!("CONTEUDO COMPLETO:\n {} \n\n\n\n\n\n\n\n\n\n", page_source);

        elements = driver.find_all(By::Css(css)).await?;
        println!("Qtd Elementos Encontrados: {:?}", elements.len());
        if elements.len() == 6 {
            println!("Encontramos exatamente 6 elementos com o seletor 'h2 > span.ng-binding': {:?}", elements);
            break;
        }

        // Verifica se o tempo máximo foi alcançado
        if elapsed >= timeout {
            println!("Tempo de espera excedido sem encontrar 6 elementos.");
            break;
        }

        sleep(interval).await;
        elapsed += interval;
    }


    println!("Procurando o seletor {}\nEncontramos {} elementos.\nSão eles: {:?}", css, elements.len(), elements);
    let page_source = driver.source().await?;
    println!("CONTEUDO COMPLETO:\n{}", page_source);

    driver.quit().await?;
    drop(chromedriver);

    println!("FINALIZANDO LotteryBtc-Crawler!");
    Ok(())
}
