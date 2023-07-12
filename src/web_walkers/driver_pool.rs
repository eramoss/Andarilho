use thirtyfour::prelude::*;

pub struct WebDriverPool {
    pub workers: Vec<WebDriver>,
}

impl WebDriverPool {
    pub async fn new(max_workers: usize) -> WebDriverResult<WebDriverPool> {
        let mut workers = Vec::with_capacity(max_workers);
        for _ in 0..workers.len() {
            // Configure Firefox options to run in headless mode.
            let mut caps = DesiredCapabilities::firefox();
            caps.add_firefox_arg("-headless")
                .expect("Cannot open Firefox without window");
            // Create a new WebDriver instance.
            let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
            workers.push(driver);
        }
        Ok(WebDriverPool { workers })
    }
}
