#[cfg(test)]
mod test {
    extern crate knock;

    use self::knock::*;

    #[test]
    fn setup_get_method_var_in_http() {
        let mut http = HTTP::new("http://rand-lang.org/").unwrap();
        let _ = http.get();

        assert!(http.method == "GET".to_string(), "should be equals");
    }

    #[test]
    fn setup_post_method_var_in_http() {
        let mut http = HTTP::new("http://rand-lang.org/").unwrap();
        let _ = http.request("POST");

        assert!(http.method == "POST".to_string(), "should be equals");
    }

    #[test]
    fn setup_request_with_body_str() {
        let mut http = HTTP::new("http://httpbin.org/post").unwrap();
        let body_str = "{\"key\":\"value\"}";
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let response = http.post()
            .header(headers)
            .body_as_str(body_str)
            .send()
            .unwrap();

        assert!(response.status == 200, "Status should be 200");
    }

    #[test]
    fn setup_request_with_body_str_second() {
        let mut http = HTTP::new("http://httpbin.org/post").unwrap();
        let body_str = "{\"key\":\"value\"}";
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let response = http.post()
            .header(headers)
            .body_as_str(body_str)
            .send()
            .unwrap();

        // httpbin returns JSON with the data in the "data" field
        assert!(response.body.contains("\"data\": \"{\\\"key\\\":\\\"value\\\"}\""), "Response should contain the sent body in data field");
    }

    #[test]
    fn get_reponse_as_string() {
        let mut http = HTTP::new("http://httpbin.org/get").unwrap();
        let response = http.get().send().unwrap();
        let string = response.as_str();

        assert!(!string.is_empty(), "Response shouldn't be empty");
    }
    #[cfg(feature = "native-tls")]
    #[test]
    fn get_tls_response() {
        let mut http = HTTP::new("https://google.com/").unwrap();
        let response = http.get().send().unwrap();
        let string = response.as_str();

        assert!(!string.is_empty(), "Response shouldn't be empty");
    }
}
