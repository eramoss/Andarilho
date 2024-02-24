use crate::web_walkers::amazon_walker::search_on_amazon;
use rocket::get;

#[get("/api/amazon/get_all/<search>")]
pub async fn get_all(search: &str) -> String {
    let results = search_on_amazon(search).await.expect("has no item");
    serde_json::json!(&results).to_string()
}
