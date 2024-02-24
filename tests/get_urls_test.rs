#[cfg(test)]
mod get_urls_test {
    use andarilho::web_walkers::*;

    #[test]
    fn test_get_url() {
        let item_name = "example item";
        let expected_url = "https://www.amazon.com.br/s?k=example%20item&ref=nb_sb_noss_1&page";
        assert_eq!(amazon_walker::get_url(item_name), expected_url);
    }

    #[test]
    fn test_get_url_with_special_characters() {
        let item_name = "special!@#$%^&*()_+ characters";
        let expected_url = "https://www.amazon.com.br/s?k=special%21%40%23%24%25%5E%26%2A%28%29_%2B%20characters&ref=nb_sb_noss_1&page";
        assert_eq!(amazon_walker::get_url(item_name), expected_url);
    }

    #[test]
    fn test_get_url_with_empty_string() {
        let item_name = "";
        let expected_url = "https://www.amazon.com.br/s?k=&ref=nb_sb_noss_1&page";
        assert_eq!(amazon_walker::get_url(item_name), expected_url);
    }
}
