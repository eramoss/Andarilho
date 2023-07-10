#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::web_walkers::RecordResults;

    #[test]
    fn serialize_record_result() {
        let result = RecordResults::new("test", "R$12.00", "5 stars", "webpage.com");

        let response_result_as_json = json!(result);
        assert_eq!(response_result_as_json.to_string(),"{\"description\":\"test\",\"price\":\"R$12.00\",\"review\":\"5 stars\",\"url\":\"webpage.com\"}");
    }

    #[test]
    fn deserialize_record_result() {
        let result = RecordResults::new("test", "R$12.00", "5 stars", "webpage.com");
        let response_result_as_json = json!(&result).to_string();
        let deserialized_result: RecordResults =
            serde_json::from_str(&response_result_as_json).unwrap();

        assert_eq!(deserialized_result.description, result.description);
        assert_eq!(deserialized_result.price, result.price);
        assert_eq!(deserialized_result.review, result.review);
        assert_eq!(deserialized_result.url, result.url);
    }
}
