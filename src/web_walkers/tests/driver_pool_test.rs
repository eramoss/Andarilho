#[cfg(test)]
mod tests_pool {

    use crate::web_walkers::driver_pool::*;
    #[tokio::test]
    async fn create_pool() {
        let pool = WebDriverPool::new(4).await.unwrap();
        assert_eq!(pool.workers.len(), 4);
        for workers in pool.workers {
            assert!(workers.status().await.unwrap().ready);
        }
    }

    #[tokio::test]
    async fn execute_with_worker() {
        let mut pool = WebDriverPool::new(4).await.unwrap();
        let driver = pool.get_driver().await.unwrap();
        driver.goto("https://www.rust-lang.org").await.unwrap();
        assert_eq!(driver.title().await.unwrap(), "Rust Programming Language");
        pool.return_driver(driver);
    }
}
