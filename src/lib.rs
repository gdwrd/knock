extern crate url;
extern crate rand;
extern crate serde_json;

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

mod err;
mod consts;

pub struct HTTP {
    pub response: Response,
    pub url: url::Url,

    method: String,
    body: HashMap<String, Data>,
    header: HashMap<String, String>,
    request_str: String,

    host: String,
    c_type: String,
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
            request_str: String::new(),

            host: host_url,
            c_type: String::new(),
            boundary: String::new(),
            response_str: String::new(),
        })
    }

    pub fn request(&mut self, method: &str) -> &mut Self {
        self.method = method.to_string();
        self
    }

    pub fn body(&mut self, data: HashMap<String, Data>) -> &mut Self {
        self.body = data;
        self
    }

    pub fn header(&mut self, data: HashMap<String, String>) -> &mut Self {
        self.header = data;
        self
    }

    pub fn send(&mut self) -> Result<Response, HttpError> {
        self.boundary = rand::thread_rng()
            .gen_ascii_chars()
            .take(32)
            .collect::<String>();
        if self.url.scheme() == "http" {
           let url = try!(self.url.host_str().ok_or(ParseError::EmptyHost));
           self.host = url.to_string();
           let port = match self.url.port() {
               Some(p) => p,
               None => DEF_PORT,
           };
           let addr = format!("{0}:{1}", url, port);
           let mut stream = try!(TcpStream::connect(addr));
           let request = try!(self.create_request());
           try!(stream.write(request.as_bytes()));
           try!(stream.read_to_string(&mut self.response_str));
           // TODO uncomplete
        } else {
           // TODO SSL Connection
        }

        Ok(Response { status: 0, header: HashMap::new(), body: self.response_str.clone() })
    }

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

        for (key, val) in header.iter() {
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

fn organize_header(header: &HashMap<String, String>, host: String) -> (HashMap<String, String>, String) {
    let mut data: HashMap<String, String> = HashMap::new();
    let mut c_type = String::new();

    if !header.is_empty() {
        for (key, val) in header.iter() {
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
