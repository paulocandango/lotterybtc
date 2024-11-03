use dotenv::dotenv;
use tokio;
use std::time::Duration;
use tokio::time::sleep;
use thirtyfour::prelude::*;
use std::process::Command;
use scraper::{Html, Selector};

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
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.set_disable_gpu()?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--window-size=1920,1080")?;

    // Inicia o WebDriver e navega até a página desejada
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.get("https://projloto.onrender.com/static/example.html").await?;
    println!("Página carregada com sucesso!");

    // Captura o HTML completo da página
    let page_source = driver.source().await?;
    println!("Conteúdo completo da página:\n{}", page_source);

    let document = Html::parse_document(&page_source);
    let concurso_selector = Selector::parse("#identity").unwrap();
    if let Some(resultado) = document.select(&concurso_selector).next() {
        // Captura o texto do concurso
        let concurso_texto = resultado.inner_html(); // ou use resultado.text() para pegar apenas o texto sem HTML
        println!("{}", concurso_texto);
    } else {
        println!("Resultado não encontrado.");
    }

    // Encerra o driver para liberar recursos
    driver.quit().await?;
    drop(chromedriver); // Encerra o processo do ChromeDriver

    println!("FINALIZANDO LotteryBtc-Crawler!");
    Ok(())
}
