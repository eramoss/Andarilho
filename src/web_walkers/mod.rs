pub mod amazon_walker;
pub mod driver_pool;
pub mod tests;

use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RecordResults {
    description: String,
    price: String,
    review: String,
    url: String,
}

impl RecordResults {
    pub fn new(description: &str, price: &str, review: &str, url: &str) -> RecordResults {
        RecordResults {
            description: description.to_string(),
            price: price.to_string(),
            review: review.to_string(),
            url: url.to_string(),
        }
    }
}
pub struct RecordTags {
    anchor_tag: WebElement,
    description_tag: WebElement,
    price_tag: WebElement,
    review_tag: Option<WebElement>,
}
///  This function assume that you have a web server of selenium running on port 4444
///  
///  # Returns
///  A `WebDriverResult` containing a `WebDriver` struct with extracted tags.
///  
///  # Example
/// ```rust
///   let driver = start_driver();
///   driver.goto("your_web_page.com");
/// ```
pub async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
    Ok(driver)
}

pub async fn search_on_amazon(item_name: &str) -> WebDriverResult<Vec<RecordResults>> {
    let driver = start_driver().await?;
    driver.goto(amazon_walker::get_url(item_name)).await?;
    let result = amazon_walker::get_all_records(&driver).await;
    driver.quit().await?;
    result
}
