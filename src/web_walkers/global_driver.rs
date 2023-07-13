use thirtyfour::prelude::*;

static mut DRIVER: Option<WebDriver> = None;

/// Retrieves the WebDriver driver.
///
/// This function returns a mutable reference to the existing WebDriver driver or starts a new driver if none exists. The WebDriver driver is maintained as a static variable.
///
/// # Returns
///
/// A result containing a mutable reference to the WebDriver driver. If successful, the reference to the driver is returned. If starting a new driver fails, an error of type WebDriverResult is returned.
///
/// # Example
///
///  ```rust
///   let driver_one = get_driver().await.unwrap();
///   let driver_two = get_driver().await.unwrap();
///
///   driver_one.goto("https://github.com").await.unwrap();
///
///   assert_eq!(
///     driver_one.title().await.unwrap(),
///     driver_two.title().await.unwrap()
///   );
///   ```
/// # Remarks
///
/// - The WebDriver driver is maintained as a static variable to ensure only one instance of the driver is used throughout the program.
/// - Make sure to use `driver.new_tab.await?` to change the window state of driver
pub async fn get_driver() -> WebDriverResult<&'static mut WebDriver> {
    let driver = unsafe { &mut DRIVER };
    if driver.is_none() {
        *driver = Some(start_driver().await.expect("unable to start a driver"));
    }

    Ok(driver.as_mut().unwrap())
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
async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
    Ok(driver)
}
