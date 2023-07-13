#[cfg(test)]
mod tests {

    // to run all tests, make sure that the your selenium server is running and accept more than 1 driver instance
    use crate::web_walkers::global_driver::*;

    #[tokio::test]
    async fn start_two_windows_same_driver() {
        let driver = get_driver().await.unwrap();

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
    }

    #[tokio::test]
    async fn get_two_drivers_as_mistake() {
        //make sure that every time you get a driver, change the page with
        //driver.new_tab()
        let driver_one = get_driver().await.unwrap();
        let driver_two = get_driver().await.unwrap();

        driver_one.goto("https://github.com").await.unwrap();

        assert_eq!(
            driver_one.title().await.unwrap(),
            driver_two.title().await.unwrap()
        );
    }
}
