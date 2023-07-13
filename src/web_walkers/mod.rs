pub mod amazon_walker;
pub mod global_driver;
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

pub async fn search_on_amazon(item_name: &str) -> WebDriverResult<Vec<RecordResults>> {
    let driver = global_driver::get_driver().await?;
    driver.goto(amazon_walker::get_url(item_name)).await?;
    let result = amazon_walker::get_all_records(&driver).await;
    result
}
