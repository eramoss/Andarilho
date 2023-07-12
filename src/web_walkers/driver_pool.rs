use std::env;
use thirtyfour::prelude::*;

static mut POOL: Option<WebDriverPool> = None;

pub async fn get_global_pool() -> WebDriverResult<&'static mut WebDriverPool> {
    let pool: &mut Option<WebDriverPool> = unsafe { &mut POOL };
    if pool.is_none() {
        let max_nodes: usize = match env::var("SE_NODE_MAX_SESSIONS") {
            Ok(n) => n
                .parse()
                .expect("cannot parse SE_NODE_MAX_SESSIONS, must be a positive integer"),
            Err(_) => {
                println!("SE_NODE_MAX_SESSIONS no declared, set to 1");
                1
            }
        };
        *pool = Some(WebDriverPool::new(max_nodes - 1).await.unwrap());
    }
    Ok(pool.as_mut().unwrap())
}
///  This function assume that you have a web server of selenium running on port 4444
///  
///  # Returns
///  A `WebDriverResult` containing a `WebDriver` struct with extracted tags.
///  
///  # Example
/// ```rust
///   let driver = start_driver();
///   driver.goto("your_web_page.com");
/// ```
/// # WARNING: UNUSED FUNCTION!!!
async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
    Ok(driver)
}

pub struct WebDriverPool {
    pub workers: Vec<WebDriver>,
}

impl WebDriverPool {
    pub async fn new(max_workers: usize) -> WebDriverResult<WebDriverPool> {
        let mut workers = Vec::with_capacity(max_workers);
        for _ in 0..max_workers {
            let driver = start_driver().await?;
            workers.push(driver);
        }
        Ok(WebDriverPool { workers })
    }

    pub fn get_driver(&mut self) -> Option<WebDriver> {
        self.workers.pop()
    }

    pub fn return_driver(&mut self, driver: WebDriver) {
        self.workers.push(driver);
    }
}
