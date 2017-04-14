extern crate url;
extern crate rand;
extern crate serde_json;
extern crate openssl;

use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use rand::Rng;
use url::{Url, ParseError};
use consts::*;
use err::HttpError;
use openssl::ssl::{SslMethod, SslConnectorBuilder};

mod err;
mod consts;

pub struct HTTP {
    pub response: Response,
    pub url: url::Url,

    method: String,
    body: HashMap<String, Data>,
    header: HashMap<String, String>,

    host: String,
    boundary: String,
    response_str: String,
}

#[derive(Debug)]
pub struct Response {
    pub status: u32,
    pub header: HashMap<String, String>,
    pub body: String,
}

pub enum Data {
    File(String),
    String(String),
}

impl HTTP {
    pub fn new(url: &str) -> Result<HTTP, HttpError> {
        let response = Response { status: 0, header: HashMap::new(), body: String::new() };
        let url = try!(Url::parse(url));
        let host_url = match url.host_str() {
            Some(url) => url.to_string(),
            None => String::new()
        };

        Ok(HTTP {
            response: response,
            url: url,

            method: String::new(),
            body: HashMap::new(),
            header: HashMap::new(),

            host: host_url,
            boundary: String::new(),
            response_str: String::new(),
        })
    }

    ///
    // GET request
    //
    // Params: &mut self (HTTP)
    //
    // Response: &mut self (HTTP)
    //
    pub fn get(&mut self) -> &mut Self {
        self.method = "GET".to_string();
        self
    }

    ///
    // POST request
    //
    // Params: &mut self (HTTP)
    //
    // Response: &mut self (HTTP)
    //
    pub fn post(&mut self) -> &mut Self {
        self.method = "POST".to_string();
        self
    }

    ///
    // PUT request
    //
    // Params: &mut self (HTTP)
    //
    // Response: &mut self (HTTP)
    //
    pub fn put(&mut self) -> &mut Self {
        self.method = "PUT".to_string();
        self
    }

    ///
    // DELETE request
    //
    // Params: &mut self (HTTP)
    //
    // Response: &mut self (HTTP)
    //
    pub fn delete(&mut self) -> &mut Self {
        self.method = "DELETE".to_string();
        self
    }

    ///
    // REQEUST request
    //
    // Params: &mut self (HTTP), method &str
    //
    // Response: &mut self (HTTP)
    //
    pub fn request(&mut self, method: &str) -> &mut Self {
        self.method = method.to_string();
        self
    }

    ///
    // Set Body to self.body
    //
    // Params: &mut self (HTTP), data HashMap<String, Data>
    //
    // Response: &mut self (HTTP)
    //
    pub fn body(&mut self, data: HashMap<String, Data>) -> &mut Self {
        self.body = data;
        self
    }

    ///
    // Set Headers to self.header
    //
    // Params: &mut self (HTTP), data HashMap<String, String>
    //
    // Response: &mut self (HTTP)
    //
    pub fn header(&mut self, data: HashMap<String, String>) -> &mut Self {
        self.header = data;
        self
    }

    ///
    // Create request, and send
    //
    // Params: &mut self (HTTP)
    //
    // Response: Result<Response, HttpError>
    //
    pub fn send(&mut self) -> Result<Response, HttpError> {
        self.boundary = rand::thread_rng()
            .gen_ascii_chars()
            .take(32)
            .collect::<String>();

        let url = try!(self.url.host_str().ok_or(ParseError::EmptyHost));
        self.host = url.to_string();
        let request = try!(self.create_request());

        if self.url.scheme() == "http" {
           let port = match self.url.port() {
               Some(p) => p,
               None    => DEF_PORT,
           };
           let addr = format!("{}:{}", url, port);
           let mut stream = try!(TcpStream::connect(addr));
           try!(stream.write(request.as_bytes()));
           try!(stream.read_to_string(&mut self.response_str));
        } else {
           let port = match self.url.port() {
               Some(p) => p,
               None    => DEF_SSL_PORT,
           };
           let addr = format!("{}:{}", url, port);
           let connector = try!(SslConnectorBuilder::new(SslMethod::tls())).build();
           let stream = try!(TcpStream::connect(addr));
           let mut stream = try!(connector.connect(&self.host, stream));

           try!(stream.write(request.as_bytes()));
           try!(stream.read_to_string(&mut self.response_str));
        }

        Ok(Response { status: 0, header: HashMap::new(), body: self.response_str.clone() })
    }

    ///
    // Create Reqeust String
    //
    // Params: &mut self (HTTP)
    //
    // Response: Result<String, HttpError>
    //
    fn create_request(&self) -> Result<String, HttpError> {
        let (mut header, c_type) = organize_header(&self.header, self.host.clone());
        let body = try!(create_body(&c_type, &self.body, header.clone(), &self.boundary));

        {
            let cl_methods = CL_METHODS.clone();
            let method_exist = cl_methods.iter().find(|&&x| x == self.method);
            match method_exist {
                Some(_) => {
                    header.insert(H_CLEN.to_string(), body.len().to_string());
                },
                None => { },
            }
        }

        let mut str = String::new();
        str += &format!("{0} {1} {2}{3}", self.method, self.url.path(), HTTP_VERSION, SEP);

        for (key, val) in &header {
            str += &format!("{}: {}{}", key, val, SEP);
        }

        Ok(format!("{}{}{}", str, SEP, body))
    }
}

fn create_body(c_type: &str, body: &HashMap<String, Data>, mut header: HashMap<String, String>, b: &str)
    -> Result<String, HttpError> {
    let mut res = String::new();

    if c_type == C_TYPE[1] {
        for (key, val) in body.iter() {
            match val {
                &Data::String(ref str) => res += &format!("{}={}", key, str),
                &Data::File(_) => continue,
            }
            if body.keys().last().unwrap() != key {
                res += &format!("&");
            }
        }
    } else if c_type == C_TYPE[2] {
        *header.get_mut(H_CTYPE).unwrap() = format!("{}; boundary={}", c_type, b);

        for (key, val) in body.iter() {
            res += &format!("--{}{}", b, SEP);
            match val {
                &Data::File(ref str) => {
                    res += &format!("Content-Disposition: form-data; name={};", key);
                    let file_name = try!(Path::new(str)
                        .file_name()
                        .ok_or(Error::new(ErrorKind::InvalidData, "wrong file path")));
                    res += &format!(" filename={0}{1}{1}", file_name.to_str().unwrap(), SEP);
                    let mut buffer = String::new();
                    let mut file = try!(File::open(str));
                    try!(file.read_to_string(&mut buffer));
                    res += &format!("{}{}", buffer, SEP);
                },
                &Data::String(ref str) => {
                    res += &format!("Content-Disposition: form-data; name={0}{1}{1}", key, SEP);
                    res += &format!("{}{}", str, SEP);
                }
            }
        }

        res += &format!("--{}--", b);
    } else {
        let mut tmp_map: HashMap<&str, &str> = HashMap::new();
        for (key, val) in body.iter() {
            match val {
                &Data::String(ref str) => {
                    tmp_map.insert(key, str);
                },
                &Data::File(_) => continue,
            }
        }

        res = try!(serde_json::to_string(&tmp_map));
    }

    Ok(res)
}

fn organize_header(header: &HashMap<String, String>, host: String)
    -> (HashMap<String, String>, String) {
    let mut data: HashMap<String, String> = HashMap::new();
    let mut c_type = String::new();

    if !header.is_empty() {
        for (key, val) in header {
            if data.contains_key(val) {
                let p_val = data.get(key).unwrap().to_string();  // Always be Some(val)
                let str = format!("{}; {}", p_val, val);
                data.insert(key.to_string(), str);
            } else {
                data.insert(key.to_string(), val.to_string());
            }
        }
    }

    if !data.contains_key(H_HOST)  { data.insert(H_HOST.to_string(), host.to_string()); }
    if !data.contains_key(H_ACCPT) { data.insert(H_ACCPT.to_string(), DEF_ACCEPT.to_string()); }
    if !data.contains_key(H_CONN)  { data.insert(H_CONN.to_string(), DEF_CONN.to_string()); }
    if data.contains_key(H_CTYPE) {
        let c_types = C_TYPE.clone();
        c_type = data.get(H_CTYPE).unwrap().to_string();
        let c_type_exist = c_types.iter().find(|&&x| x == c_type);
        match c_type_exist {
            None => {
                data.insert(H_CTYPE.to_string(), C_TYPE[0].to_string());
            },
            Some(_) => { },
        }
    } else {
        data.insert(H_CTYPE.to_string(), C_TYPE[0].to_string());
    }

    (data, c_type)
}
