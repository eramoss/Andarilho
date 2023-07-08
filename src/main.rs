pub mod web_walkers;
#[tokio::main]
async fn main() {
    let results = web_walkers::amazon_walker::get_all_records("test")
        .await
        .unwrap();
    dbg!(results);
}
