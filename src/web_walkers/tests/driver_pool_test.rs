#[cfg(test)]
mod tests_pool {
    use crate::web_walkers::driver_pool::WebDriverPool;
    #[tokio::test]
    async fn create_pool() {
        let pool = WebDriverPool::new(5).await.unwrap();
        for workers in pool.workers {
            assert_eq!(
                workers.status().await.unwrap().message,
                "Selenium Grid is ready"
            );
        }
    }
}
