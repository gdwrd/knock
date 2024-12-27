#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate url;
extern crate rand;
extern crate serde_json;
#[cfg(feature = "native-tls")]
extern crate native_tls;

use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use rand::{Rng, distributions::Alphanumeric};
use url::{Url, ParseError};
use consts::*;
use err::HttpError;
use response::*;
#[cfg(feature = "native-tls")]
use native_tls::TlsConnector;

mod err;
mod consts;
pub mod response;

/// HTTP struct
///
/// ```rust
/// extern crate knock;
///
/// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
/// ```
///
pub struct HTTP {
    pub response: Response,
    pub url: url::Url,

    pub method: String,
    pub body: HashMap<String, Data>,
    pub header: HashMap<String, String>,
    body_str: String,

    danger_accept_invalid_certs: bool,

    host: String,
    boundary: String,
    response_str: String,
}

pub enum Data {
    File(String),
    String(String),
}

impl HTTP {
    /// HTTP struct instance
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = match knock::HTTP::new("https://example.com/api/date") {
    ///     Ok(http) => http,
    ///     Err(err) => panic!(err)
    /// };
    /// ```
    ///
    pub fn new(url: &str) -> Result<HTTP, HttpError> {
        let response = Response {
            status: 0,
            header: HashMap::new(),
            body: String::new(),
        };
        let url = Url::parse(url)?;
        let host_url = match url.host_str() {
            Some(url) => url.to_string(),
            None => String::new(),
        };

        Ok(HTTP {
            response: response,
            url: url,

            danger_accept_invalid_certs: false,

            method: String::new(),
            body: HashMap::new(),
            header: HashMap::new(),
            body_str: String::new(),

            host: host_url,
            boundary: String::new(),
            response_str: String::new(),
        })
    }

    /// How to send simple GET request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
    /// http.get().send();
    /// ```
    ///
    pub fn get(&mut self) -> &mut Self {
        self.method = "GET".to_string();
        self
    }

    /// How to send simple POST request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
    /// http.post().send();
    /// ```
    ///
    pub fn post(&mut self) -> &mut Self {
        self.method = "POST".to_string();
        self
    }

    /// How to send simple PUT request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
    /// http.put().send();
    /// ```
    ///
    pub fn put(&mut self) -> &mut Self {
        self.method = "PUT".to_string();
        self
    }

    /// How to send simple DELETE request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
    /// http.delete().send();
    /// ```
    ///
    pub fn delete(&mut self) -> &mut Self {
        self.method = "DELETE".to_string();
        self
    }

    /// Disable SSL validation before performing a GET request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/date").unwrap();
    /// http.danger_accept_invalid_certs(true).get().send();
    /// ```
    ///
    pub fn danger_accept_invalid_certs(&mut self, danger_accept_invalid_certs: bool) -> &mut Self {
        self.danger_accept_invalid_certs = danger_accept_invalid_certs;
        self
    }

    /// Send custom Request
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/data").unwrap();
    /// http.request("OPTIONS").send();
    /// ```
    ///
    pub fn request(&mut self, method: &str) -> &mut Self {
        self.method = method.to_string();
        self
    }

    /// Send Body data as HashMap<String, Data>
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/data").unwrap();
    /// let mut body: HashMap<String, knock::Data> = HashMap::new();
    /// body.insert("key".to_string(), knock::Data::String("value".to_string()));
    ///
    /// http.post().body(body).send();
    /// ```
    ///
    pub fn body(&mut self, data: HashMap<String, Data>) -> &mut Self {
        self.body = data;
        self
    }

    /// You also can use body_as_str function for sending body as String
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/data").unwrap();
    /// http.post().body_as_str("{\"key\": \"value\"}").send();
    /// ```
    /// But you need to set Content-Type in header,
    /// default Content-Type
    pub fn body_as_str(&mut self, data: &str) -> &mut Self {
        self.body_str = data.to_string();
        self
    }

    /// Send Body data as HashMap<String, Data>
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/data").unwrap();
    /// let mut body: HashMap<String, knock::Data> = HashMap::new();
    /// let mut header: HashMap<String, String> = HashMap::new();
    ///
    /// body.insert("key".to_string(), knock::Data::String("value".to_string()));
    /// header.insert("Content-Type".to_string(), "application/json".to_string());
    ///
    /// http.post().body(body).header(header).send();
    /// ```
    ///
    pub fn header(&mut self, data: HashMap<String, String>) -> &mut Self {
        self.header = data;
        self
    }

    /// Send Body data as HashMap<String, Data>
    ///
    /// ```rust
    /// extern crate knock;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut http = knock::HTTP::new("https://example.com/api/data").unwrap();
    /// let mut body: HashMap<String, knock::Data> = HashMap::new();
    ///
    /// body.insert("key".to_string(), knock::Data::String("value".to_string()));
    ///
    /// match http.post().body(body).send() {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err)
    /// };
    /// ```
    ///
    pub fn send(&mut self) -> Result<Response, HttpError> {
        self.boundary = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        let url = self.url.host_str().ok_or(ParseError::EmptyHost)?;
        self.host = url.to_string();
        let request = self.create_request()?;
        let response;

        if self.url.scheme() == "http" {
            let port = match self.url.port() {
                Some(p) => p,
                None => DEF_PORT,
            };
            let addr = format!("{}:{}", url, port);
            let mut stream = TcpStream::connect(addr)?;
            stream.write_all(request.as_bytes())?;
            stream.read_to_string(&mut self.response_str)?;
        } else {
            self.response_str = self.tls_transport(request, url)?;
        }

        response = self.response_str.clone();
        let resp = Response::new(response).unwrap();
        Ok(resp)
    }

    #[cfg(feature = "native-tls")]
    fn tls_transport(&self, request: String, url: &str) -> Result<String, HttpError> {
        let port = match self.url.port() {
            Some(p) => p,
            None => DEF_SSL_PORT,
        };
        let addr = format!("{}:{}", url, port);

        let connector = match self.danger_accept_invalid_certs {
            true  => TlsConnector::builder().danger_accept_invalid_certs(true).build()?,
            false => TlsConnector::builder().build()?,
        };
        let stream = TcpStream::connect(addr)?;
        let mut stream = connector.connect(&self.host, stream)?;

        stream.write_all(request.as_bytes())?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        Ok(buf)
    }

    #[cfg(not(feature = "native-tls"))]
    fn tls_transport(&self, _request: String, _url: &str) -> Result<String, HttpError> {
        Err(HttpError::MissingFeature(
            "Lib not compiled with feature native-tls active".into(),
        ))
    }
    /// Create Reqeust String
    ///
    /// Params: &mut self (HTTP)
    ///
    /// Response: Result<String, HttpError>
    ///
    fn create_request(&self) -> Result<String, HttpError> {
        let (mut header, c_type) = organize_header(&self.header, &self.host);

        let body = if self.body_str.is_empty() {
            create_body(&c_type, &self.body, header.clone(), &self.boundary)?
        } else {
            self.body_str.clone()
        };

        {
            let cl_methods = CL_METHODS;
            let method_exist = cl_methods.iter().find(|&&x| x == self.method);
            if method_exist.is_some() {
                header.insert(H_CLEN.to_string(), body.len().to_string());
            }
        }

        let path = match self.url.query() {
            Some(q) => format!("{}?{}", self.url.path(), q),
            None => self.url.path().to_string(),
        };
        let mut str = String::new();
        str += &format!("{} {} {}{}", self.method, path, HTTP_VERSION, SEP);

        for (key, val) in &header {
            str += &format!("{}: {}{}", key, val, SEP);
        }

        Ok(format!("{}{}{}", str, SEP, body))
    }
}

/// Create Body for request
///
/// Params: `c_type`: &str, body: &`HashMap`<String, Data>, mut header: `HashMap`<String, String>, b: &str
///
/// Response: Result<String, `HttpError`>
///
fn create_body(
    c_type: &str,
    body: &HashMap<String, Data>,
    mut header: HashMap<String, String>,
    b: &str,
) -> Result<String, HttpError> {
    let mut res = String::new();

    if c_type == C_TYPE[1] {
        for (key, val) in body.iter() {
            match *val {
                Data::String(ref str) => res += &format!("{}={}", key, str),
                Data::File(_) => continue,
            }
            if body.keys().last().unwrap() != key {
                res += "&";
            }
        }
    } else if c_type == C_TYPE[2] {
        *header.get_mut(H_CTYPE).unwrap() = format!("{}; boundary={}", c_type, b);

        for (key, val) in body.iter() {
            res += &format!("--{}{}", b, SEP);
            match *val {
                Data::File(ref str) => {
                    res += &format!("Content-Disposition: form-data; name={};", key);
                    let file_name = Path::new(str).file_name().ok_or_else(|| {
                        Error::new(ErrorKind::InvalidData, "wrong file path")
                    })?;
                    res += &format!(" filename={0}{1}{1}", file_name.to_str().unwrap(), SEP);
                    let mut buffer = String::new();
                    let mut file = File::open(str)?;
                    file.read_to_string(&mut buffer)?;
                    res += &format!("{}{}", buffer, SEP);
                }
                Data::String(ref str) => {
                    res += &format!("Content-Disposition: form-data; name={0}{1}{1}", key, SEP);
                    res += &format!("{}{}", str, SEP);
                }
            }
        }

        res += &format!("--{}--", b);
    } else {
        let mut tmp_map: HashMap<&str, &str> = HashMap::new();
        for (key, val) in body.iter() {
            match *val {
                Data::String(ref str) => {
                    tmp_map.insert(key, str);
                }
                Data::File(_) => continue,
            }
        }

        res = serde_json::to_string(&tmp_map)?;
    }

    Ok(res)
}

/// Update Header
///
/// Params: mut header: `HashMap`<String, String>, host: String
///
/// Response: (Result<String, `HttpError`>, String)
///
fn organize_header(
    header: &HashMap<String, String>,
    host: &str,
) -> (HashMap<String, String>, String) {
    let mut data: HashMap<String, String> = HashMap::new();
    let mut c_type = String::new();

    if !header.is_empty() {
        for (key, val) in header {
            if data.contains_key(val) {
                let p_val = data[key].to_string(); // Always be Some(val)
                let str = format!("{}; {}", p_val, val);
                data.insert(key.to_string(), str);
            } else {
                data.insert(key.to_string(), val.to_string());
            }
        }
    }

    if !data.contains_key(H_HOST) {
        data.insert(H_HOST.to_string(), host.to_string());
    }
    if !data.contains_key(H_ACCPT) {
        data.insert(H_ACCPT.to_string(), DEF_ACCEPT.to_string());
    }
    if !data.contains_key(H_CONN) {
        data.insert(H_CONN.to_string(), DEF_CONN.to_string());
    }
    if data.contains_key(H_CTYPE) {
        let c_types = C_TYPE;
        c_type = data[H_CTYPE].to_string();
        let c_type_exist = c_types.iter().find(|&&x| x == c_type);
        match c_type_exist {
            None => {
                data.insert(H_CTYPE.to_string(), C_TYPE[0].to_string());
            }
            Some(_) => {}
        }
    } else {
        data.insert(H_CTYPE.to_string(), C_TYPE[0].to_string());
    }

    (data, c_type)
}

#[cfg(test)]
mod tests {
    use super::HTTP;

    #[test]
    fn test_query_params() {
        let mut http = HTTP::new("http://moo.com/?foo=bar").unwrap();
        let expected = "GET /?foo=bar".to_string();
        assert_eq!(
            http.get().create_request().unwrap()[0..expected.len()],
            expected
        );
    }
}
