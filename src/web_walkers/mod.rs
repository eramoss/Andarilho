use thirtyfour::prelude::*;

pub mod amazon_walker;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
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
