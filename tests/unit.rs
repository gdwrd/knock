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
        let mut http = HTTP::new("http://www.mocky.io/v2/58f48af0100000b60f68cad8").unwrap();
        let body_str = "{\"key\":\"value\"}";
        let response = http.post().body_as_str(body_str).send().unwrap();

        assert!(response.status == 200, "Status should be 200");
    }

    #[test]
    fn setup_request_with_body_str_second() {
        let mut http = HTTP::new("http://www.mocky.io/v2/58f48af0100000b60f68cad8").unwrap();
        let body_str = "{\"key\":\"value\"}";
        let response = http.post().body_as_str(body_str).send().unwrap();

        assert!(response.body == body_str);
    }

    #[test]
    fn get_reponse_as_string() {
        let mut http = HTTP::new("http://www.mocky.io/v2/58f48af0100000b60f68cad8").unwrap();
        let response = http.get().send().unwrap();
        let string = response.as_str();

        assert!(!string.is_empty(), "Response shouldn't be empty");
    }
}
