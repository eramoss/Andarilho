use thirtyfour::prelude::*;

pub mod amazon_walker;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let caps = DesiredCapabilities::firefox();

    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://localhost:4444/", caps).await?;
    Ok(driver)
}

pub async fn search_on_amazon(
    item_name: &str,
) -> WebDriverResult<Vec<amazon_walker::RecordResults>> {
    let driver = start_driver().await?;
    driver.goto(amazon_walker::get_url(item_name)).await?;
    let result = amazon_walker::get_all_records(&driver).await;
    driver.quit().await?;
    result
}
