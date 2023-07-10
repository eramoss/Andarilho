use super::*;

/// This function takes an `item_name` as input and returns a formatted URL string.
/// # Arguments
/// * `item_name` - A string slice (`&str`) representing the name of the item to be searched on Amazon.
/// # Returns
/// A `String` containing the formatted URL.
/// # Example
/// ```
/// let item_name = "example item";
/// let url = get_url(item_name);
/// println!("{}", url);
/// ```
/// Output:
/// ```
/// https://www.amazon.com.br/s?k=example+item&ref=nb_sb_noss_1&page
/// ```
/// In this example, the `item_name` is "example item". The function replaces any spaces in the item name with a plus sign (+)
/// and creates a URL string using the formatted URL pattern: `https://www.amazon.com.br/s?k={item_name}&ref=nb_sb_noss_1&page{}`.
/// The resulting URL is then returned by the function.
pub fn get_url(item_name: &str) -> String {
    let item_name = item_name.replace(' ', "+");
    let url = format!(
        "https://www.amazon.com.br/s?k={}&ref=nb_sb_noss_1",
        item_name
    );
    url + "&page"
}

/// Asynchronously extracts information from the provided `RecordTags` and returns a `RecordResults` struct.
///
/// # Arguments
///
/// * `record_tags` - A `RecordTags` struct containing the extracted tags from an item's web element.
///
/// # Returns
///
/// A `WebDriverResult` containing a `RecordResults` struct with extracted information.
///
/// # Example
///
/// ```rust
/// let tags = /* obtain RecordTags */;
/// let results = extract_info_from_tags(tags).await?;
/// // Make use of the extracted information
/// # Ok::<(), WebDriverError>(())
/// ```
///
/// In this example, the `tags` is a `RecordTags` struct containing various extracted tags from an item's web element. The function
/// extracts information such as the item description, price, URL, and review. It constructs a `RecordResults` struct with this
/// information and returns it as a result.
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

/// Asynchronously extracts various tags from an item's web element and returns a `RecordTags` struct.
///
/// # Arguments
///
/// * `item` - A `WebElement` representing the item's web element to extract tags from.
///
/// # Returns
///
/// A `WebDriverResult` containing a `RecordTags` struct with extracted tags.
///
/// # Example
///
/// ```rust
/// let item_element = /* obtain WebElement */;
/// let tags = extract_tags(item_element).await?;
/// // Make use of the extracted tags
/// # Ok::<(), WebDriverError>(())
/// ```
///
/// In this example, the `item_element` is the web element representing an item. The function finds various tags within the item element,
/// such as the `h2` tag, `a` tag, `span` tag, `.a-offscreen` class tag, and `.a-icon-alt` class tag. It constructs a `RecordTags` struct
/// containing these tags and returns it as a result.
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

/// Asynchronously extracts a record from the provided `WebElement` by extracting tags and information, and returns it as a `RecordResults`.
///
/// # Arguments
///
/// * `item` - A `WebElement` representing the item's web element to extract the record from.
///
/// # Returns
///
/// A `WebDriverResult` containing a `RecordResults` struct representing the extracted record.
///
/// # Example
///
/// ```rust
/// let item_element = /* obtain WebElement */;
/// let record = extract_record(item_element).await?;
/// // Make use of the extracted record
/// # Ok::<(), WebDriverError>(())
/// ```
///
/// In this example, the `item_element` is the web element representing an item. The function first extracts tags from the item element
/// using the `extract_tags` function. It then extracts information from the extracted tags using the `extract_info_from_tags` function.
/// The resulting `RecordResults` struct represents the extracted record and is returned as a result.
pub async fn extract_record(item: WebElement) -> WebDriverResult<RecordResults> {
    let tags = extract_tags(item).await?;
    extract_info_from_tags(tags).await
}

/// Asynchronously gets all records from the web page using the provided WebDriver and returns them as a vector of `RecordResults`.
///
/// # Arguments
///
/// * `driver` - A reference to the WebDriver instance used for interacting with the web page.
///
/// # Returns
///
/// A `WebDriverResult` containing a vector of `RecordResults` representing the extracted records from the web page.
///
/// # Example
///
/// ```rust
/// let driver = /* obtain WebDriver instance */;
/// let records = get_all_records(&driver).await?;
/// // Make use of the extracted records
/// # Ok::<(), WebDriverError>(())
/// ```
///
/// In this example, the `driver` is a reference to the WebDriver instance used for interacting with the web page. The function finds
/// all the search results on the web page by locating the divs with the attribute `data-component-type="s-search-result"`. It then
/// iterates through each search result, extracts the record using the `extract_record` function, and adds it to the vector of `RecordResults`.
/// The final vector of records is returned as a result.
pub async fn get_all_records(driver: &WebDriver) -> WebDriverResult<Vec<RecordResults>> {
    //find all s-search results on web page
    //when search something in amazon, they return all items in these divs
    let search_results = driver
        .find_all(By::Css("div[data-component-type=\"s-search-result\"]"))
        .await?;

    let mut records: Vec<RecordResults> = vec![];

    // loop through all the records and extract the results
    for item in search_results {
        let record = match extract_record(item).await {
            Ok(record) => record,
            Err(_) => RecordResults::new("", "", "", ""),
        };
        records.push(record);
    }

    Ok(records)
}
