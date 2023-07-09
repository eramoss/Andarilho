pub mod web_walkers;
#[tokio::main]
async fn main() {
    web_walkers::amazon_walker::get_all_records("alo")
        .await
        .expect("");
}
