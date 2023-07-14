#[cfg(test)]
mod tests_pool {
    use crate::web_walkers::driver_pool::*;

    const POOL_SIZE: usize = 1;
    #[tokio::test]
    async fn create_pool() {
        // Init pool or get it from `get_global_pool()`;
        init_global_pool().await;
        let pool = get_global_pool().await.unwrap();

        assert_eq!(pool.workers.len(), POOL_SIZE);

        for workers in &pool.workers {
            assert!(workers.status().await.unwrap().ready);
        }
    }

    #[tokio::test]
    async fn execute_with_worker() {
        // Init pool or get it from `get_global_pool()`;
        init_global_pool().await;
        let pool = get_global_pool().await.expect("Cannot get global pool");

        // Get a driver instance of pool to use
        let driver = pool.get_driver().await.unwrap();

        driver.goto("https://www.rust-lang.org").await.unwrap();
        assert_eq!(driver.title().await.unwrap(), "Rust Programming Language");

        //after use return driver for pool
        pool.return_driver(driver);
    }
}
