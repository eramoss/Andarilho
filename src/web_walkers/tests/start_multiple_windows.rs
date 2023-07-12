#[cfg(test)]
mod tests {
    use crate::web_walkers::driver_pool::*;

    #[tokio::test]
    async fn start_two_windows_same_driver() {
        let pool = get_global_pool().await.unwrap();
        let driver = pool.get_driver().unwrap();

        let handle = driver.new_tab().await.unwrap();
        driver.switch_to_window(handle).await.unwrap();
        driver.goto("https://www.rust-lang.org").await.unwrap();
        driver.set_window_name("rust").await.unwrap();

        let handle = driver.new_tab().await.unwrap();
        driver.switch_to_window(handle).await.unwrap();
        driver.goto("https://github.com").await.unwrap();
        driver.set_window_name("github").await.unwrap();

        driver.switch_to_named_window("github").await.unwrap();
        let title_github = driver.title().await.unwrap();

        driver.switch_to_named_window("rust").await.unwrap();
        let title_rust = driver.title().await.unwrap();

        assert_ne!(title_rust, title_github);
        pool.return_driver(driver);
    }
}
