#[cfg(test)]
mod tests {
    use thirtyfour::By;

    use andarilho::wd_pool::get_global_pool;

    #[tokio::test]
    async fn test_get_element_as_string() {
        let pool = get_global_pool().await.unwrap();
        let driver = pool.get_driver().await.unwrap();

        driver.goto("https://www.rust-lang.org/").await.unwrap();
        let element = driver.find(By::Tag("header")).await.unwrap();
        let element_as_string = element.inner_html().await.unwrap();

        let html_header_rust_pag = "\n  <div class=\"flex flex-column flex-row-l\">\n    <div class=\"w-70-l mw8-l\">\n      <h1>Rust</h1>\n      <h2 class=\"mt4 mb0 f2 f1-ns\">\n        A language empowering everyone <br class=\"dn db-ns\"> to build reliable and efficient software.\n      </h2>\n    </div>\n    <div class=\"w-30-l flex-column pl0-l pr0-l pl3 pr3\">\n      <a class=\"button button-download ph4 mt0 w-100\" href=\"/learn/get-started\">\n        Get Started\n      </a>\n      <p class=\"tc f3 f2-l mt3\">\n        <a href=\"https://blog.rust-lang.org/2023/07/13/Rust-1.71.0.html\" class=\"download-link\">Version 1.71.0</a>\n      </p>\n    </div>\n  </div>\n";

        assert_eq!(element_as_string, html_header_rust_pag);
    }
}
