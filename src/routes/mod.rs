use super::web_walkers::*;

#[get("/amazon/get/<search>")]
pub async fn index(search: &str) -> String {
    let results = get_records_from_amazon(search).await.expect("has no item");
    serde_json::json!(&results).to_string()
}
