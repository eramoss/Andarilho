use super::*;
use serde::Serialize;
pub fn get_url(item_name: &str) -> String {
    let item_name = item_name.replace(' ', "+");
    let url = format!(
        "https://www.amazon.com.br/s?k={}&ref=nb_sb_noss_1",
        item_name
    );
    url + "&page{}"
}
#[allow(dead_code)]
#[derive(Serialize)]
pub struct RecordResults {
    description: String,
    price: String,
    review: String,
    url: String,
}
struct RecordTags {
    anchor_tag: WebElement,
    description_tag: WebElement,
    price_tag: WebElement,
    review_tag: Option<WebElement>,
}

async fn extract_tags(item: WebElement) -> WebDriverResult<RecordTags> {
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

    Ok(RecordTags {
        anchor_tag,
        description_tag,
        price_tag,
        review_tag,
    })
}

async fn extract_info_from_tags(record_tags: RecordTags) -> WebDriverResult<RecordResults> {
    let description = record_tags.description_tag.text().await?;

    let price = record_tags
        .price_tag
        .inner_html()
        .await?
        .replace("&nbsp;", "");

    let mut url = "https://www.amazon.com.br/".to_string();
    url.push_str(
        record_tags
            .anchor_tag
            .attr("href")
            .await?
            .expect("The item must have a link")
            .as_str(),
    );

    let mut review = String::new();
    if record_tags.review_tag.is_some() {
        review = match record_tags.review_tag.unwrap().inner_html().await {
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

pub async fn extract_record(item: WebElement) -> WebDriverResult<RecordResults> {
    let tags = extract_tags(item).await?;
    extract_info_from_tags(tags).await
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
