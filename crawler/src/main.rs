use dotenv::dotenv;
use tokio;
use std::time::Duration;
use tokio::time::sleep;
use thirtyfour;
use thirtyfour::prelude::*;
use std::process::{Command, Child};

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    println!("INICIANDO LotteryBtc-Crawler!");
    dotenv().ok();

    let chromedriver_path = dotenv::var("CHROMEDRIVER_PATH").unwrap_or_else(|_| "chromedriver".to_string());
    println!("CHROMEDRIVER_PATH: {}", chromedriver_path);

    // Inicia o ChromeDriver
    let mut chromedriver = Command::new(chromedriver_path)
        .arg("--port=9515")
        .spawn()
        .expect("Falha ao iniciar o ChromeDriver");

    // Aguarda um tempo para garantir que o ChromeDriver esteja pronto
    sleep(Duration::from_secs(2)).await;

    // Configura o WebDriver para o Chrome headless
    let mut caps = DesiredCapabilities::chrome();
    caps.set_disable_gpu()?;
    caps.add_arg("--headless")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;

    // Inicia o ChromeDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.get("https://projloto.onrender.com/static/example.html").await?;
    println!("Página carregada com sucesso!");

    let page_source = driver.page_source().await?;
    println!("Conteúdo completo da página:\n{}", page_source);

    driver.quit().await?;

    println!("FINALIZANDO LotteryBtc-Crawler!");
    Ok(())
}
