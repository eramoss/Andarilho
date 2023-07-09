use thirtyfour::prelude::*;

pub mod amazon_walker;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let caps = DesiredCapabilities::firefox();

    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://localhost:4444/", caps).await?;
    Ok(driver)
}

pub async fn get_records_from_amazon(
    item_name: &str,
) -> WebDriverResult<Vec<amazon_walker::RecordResults>> {
    let driver = start_driver().await?;
    driver.goto(amazon_walker::get_url(item_name)).await?;

    let search_results = driver
        .find_all(By::Css("div[data-component-type=\"s-search-result\"]"))
        .await?;

    let mut records: Vec<amazon_walker::RecordResults> = vec![];

    for item in search_results {
        let record = match amazon_walker::extract_record(item).await {
            Ok(record) => record,
            Err(_) => amazon_walker::RecordResults::new("", "", "", ""),
        };
        records.push(record);
    }

    driver.quit().await?;
    Ok(records)
}
