use config::{Config, File};
use playwright::Playwright;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Configuration {
    pub email: String,
    pub password: String,
}

#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    let config: Configuration = Config::builder().add_source(File::with_name("config.json")).build().expect("[ERROR] config.json not found or invalid.").try_deserialize::<Configuration>().expect("[ERROR] Can't deserialize.");

    let playwright = Playwright::initialize().await?;
    let chromium = playwright.chromium();
    playwright.prepare()?; // Install browsers

    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    page.goto_builder("https://planetcampus.wifirst.net/connect")
        .goto()
        .await?;

    page.fill_builder("input[name='email']", &*config.email)
        .fill()
        .await?;
    page.fill_builder("input[name='password']", &*config.email)
        .fill()
        .await?;

    let submit_buttons = page.query_selector_all("button[type='submit']").await?;
    if submit_buttons.len() >= 2 {
        submit_buttons
            .get(1)
            .unwrap()
            .click_builder()
            .click()
            .await?;
        println!("Clicked the second submit button!");
    } else {
        println!("Couldn't find enough submit buttons!");
    }

    page.query_selector_all("button[type='button']")
        .await?
        .get(1)
        .unwrap()
        .click_builder()
        .click()
        .await?;

    browser.close().await?;

    Ok(())
}
