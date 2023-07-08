#[allow(unused_imports)]
use regex::Regex;
use serde_json::json;
use thirtyfour::prelude::*;

fn get_url(search_name: &str) -> String {
    // make a template of amazon url to replace from the user input
    let template = "https://www.amazon.com/s?k={}&ref=nb_sb_noss_1";
    let search_name = search_name.replace(" ", "+");

    let url = template.replace("{}", search_name.as_str());
    let url = url + "&page{}";

    return url;
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct RecordsResults {
    // Just a result of function to extract records
    description: String,
    price: String,
    review_count: String,
    url: String,
}

async fn extract_record(
    item: WebElement,
) -> Result<RecordsResults, thirtyfour::error::WebDriverError> {
    Ok(RecordsResults {
        description: String::from("a"),
        price: String::from("a"),
        review_count: String::from("a"),
        url: String::from("a"),
    })
}

pub async fn get_all_records(search_name: &str) -> Result<Vec<RecordsResults>, WebDriverError> {
    let mut caps = DesiredCapabilities::firefox();
    // Disable Firefox browser from automatically opening.
    caps.insert(
        "moz:firefoxOptions".to_string(),
        json!({
            "args": ["-headless"]
        }),
    );
    /*
    this code assumes that you have a web driver running in the background
     */

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let mut records: Vec<RecordsResults> = vec![];
    let url = get_url(search_name);

    /*amount of pages to search*/
    for page in 1..4 {
        driver
            .goto(&url.replace("{}", page.to_string().as_str()))
            .await?;
        let results = driver.find_all(By::ClassName("s-result-item")).await?;

        for item in results {
            let record = extract_record(item).await;
            if record.is_ok() {
                records.push(record.unwrap());
            }
        }
    }
    driver.quit().await?;
    Ok(records)
}

#[test]
fn is_valid_amazon_url() {
    // Define a regular expression pattern for matching Amazon URLs
    let pattern = Regex::new(r"^https?://(www\.)?amazon\.[a-z]{2,3}/.*$").unwrap();
    let url = &get_url("testing search").replace("{}", "1");
    // Check if the provided URL matches the pattern
    println!("{}", url);
    assert!(pattern.is_match(url));
}

#[test]
fn is_valid_item() {}
