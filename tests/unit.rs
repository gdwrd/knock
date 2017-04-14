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
}
