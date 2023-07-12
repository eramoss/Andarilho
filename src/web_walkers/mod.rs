pub mod amazon_walker;
pub mod driver_pool;
pub mod tests;

use self::driver_pool::WebDriverPool;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use tokio::sync::Mutex;

lazy_static! {
    static ref POOL: Mutex<Option<WebDriverPool>> = Mutex::new(None);
}

pub async fn start_global_pool() -> WebDriverResult<bool> {
    let mut pool = POOL.lock().await;
    if pool.is_none() {
        *pool = Some(WebDriverPool::new(4).await.unwrap());
    }
    Ok(true)
}

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

pub async fn search_on_amazon(item_name: &str) -> WebDriverResult<Vec<RecordResults>> {
    let mut pool = POOL.lock().await;
    let driver = pool.as_mut().unwrap().get_driver().unwrap();

    driver.goto(amazon_walker::get_url(item_name)).await?;
    let result = amazon_walker::get_all_records(&driver).await;

    pool.as_mut().unwrap().return_driver(driver);
    result
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
/// # WARNING: UNUSED FUNCTION!!!
pub async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
    Ok(driver)
} /////////////////////////
