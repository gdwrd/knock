use std::collections::HashMap;
use err::HttpError;
use consts::*;

#[derive(Debug)]
pub struct Response {
    pub status: u32,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(str: String) -> Result<Response, HttpError> {
        let h_str;
        let mut body = String::new();
        let mut header: HashMap<String, String> = HashMap::new();
        let mut status = 0;

        if str.contains(SEP) {
            let data: Vec<&str> = str.split(&format!("{0}{0}", SEP)).collect();
            body = data[1].to_string();
            h_str = data[0].to_string();
        } else {
            h_str = str;
        }

        let tmp_vec: Vec<&str> = h_str.split(SEP).collect();
        let head = tmp_vec[0];

        if head.contains("HTTP/1") {
            let vec_head: Vec<&str> = head.split(' ').collect();
            status = try!(vec_head[1].parse::<u32>());
        }

        for item in &tmp_vec {
            if item.contains(": ") {
                let tmp_vec: Vec<&str> = item.split(": ").collect();
                let k = tmp_vec[0];
                let v = tmp_vec[1];
                if header.contains_key(k) {
                    let value;
                    {
                        value = header[k].to_string();
                    }
                    header.insert(k.to_string(), format!("{}; {}", value, v));
                } else {
                    header.insert(k.to_string(), v.to_string());
                }
            }
        }

        Ok(Response {
               status: status,
               header: header,
               body: body,
           })
    }
}
