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
    url + "&page{}"
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct RecordResults {
    description: String,
    price: String,
    review: String,
    url: String,
}

async fn extract_record(item: WebElement) -> WebDriverResult<RecordResults> {
    // find the items as a tag and extract the record
    let title_item_tag = item.find_all(By::Css("h2")).await?;
    let anchor_tag = title_item_tag[title_item_tag.len() - 1]
        .find(By::Css("a"))
        .await?;
    let description_tag = anchor_tag.find(By::Css("span")).await?;
    let price_tag = item.find(By::Css(".a-offscreen")).await?;
    let review_tag = match item.find(By::Css(".a-icon-alt")).await {
        Ok(tag) => Some(tag),
        Err(_) => None,
    };

    let description = description_tag.text().await?;
    let price = price_tag.inner_html().await?.replace("&nbsp;", "");

    let url_template = "https://www.amazon.com.br/";
    let url = url_template.to_string()
        + anchor_tag
            .attr("href")
            .await?
            .expect("The item must have a link")
            .as_str();

    let mut review = String::new();
    if review_tag.is_some() {
        review = match review_tag.unwrap().inner_html().await {
            Ok(review) => review,
            Err(_) => String::from(""),
        };
    }

    Ok(RecordResults {
        description,
        price,
        review,
        url,
    })
}

pub async fn get_all_records(item_name: &str) -> WebDriverResult<Vec<RecordResults>> {
    let driver = start_driver().await?;
    driver.goto(get_url(item_name)).await?;
    let search_results = driver
        .find_all(By::Css("div[data-component-type=\"s-search-result\"]"))
        .await?;

    let mut records: Vec<RecordResults> = vec![];
    for item in search_results {
        let record = match extract_record(item).await {
            Ok(record) => record,
            Err(_) => RecordResults {
                description: String::from(""),
                price: String::from(""),
                review: String::from(""),
                url: String::from(""),
            },
        };
        records.push(record);
    }

    driver.quit().await?;
    Ok(records)
}
