use std::path::Path;

use thirtyfour::prelude::*;

async fn start_driver() -> Result<WebDriver, WebDriverError> {
    // Configure Firefox options to run in headless mode.
    let caps = DesiredCapabilities::firefox();

    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://localhost:4444/", caps).await?;
    Ok(driver)
}

fn get_url(item_name: &str) -> String {
    let item_name = item_name.replace(' ', "+");
    let url = format!(
        "https://www.amazon.com.br/s?k={}&ref=nb_sb_noss_1",
        item_name
    );
    url + "&page"
}
#[allow(dead_code)]
pub struct RecordResults {
    description: String,
    price: String,
    review: String,
    url: String,
}

fn extract_record(item: WebElement) -> WebDriverResult<()> {
    Ok(())
}

pub async fn get_all_records(item_name: &str) -> WebDriverResult<()> {
    let driver = start_driver().await?;
    driver.goto(get_url(item_name)).await?;
    let search_results = driver
        .find_all(By::Css("div[data-component-type=\"s-search-result\"]"))
        .await?;

    for item in search_results {}

    driver.quit().await?;
    Ok(())
}
