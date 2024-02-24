pub mod routes;
pub mod wd_pool;
pub mod web_walkers;

use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RecordResults {
    pub description: String,
    pub price: String,
    pub review: String,
    pub url: String,
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
    pub anchor_tag: WebElement,
    pub description_tag: WebElement,
    pub price_tag: WebElement,
    pub review_tag: Option<WebElement>,
}
