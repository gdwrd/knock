pub const HTTP_VERSION: &'static str = "HTTP/1.1";
pub const CL_METHODS: [&'static str; 2] = ["POST", "PUT"];
pub const C_TYPE: [&'static str; 3] = [
    "application/json",
    "application/x-www-form-urlencoded",
    "multipart/form-data",
];
pub const SEP: &'static str = "\r\n";

pub const DEF_PORT: u16 = 80;
#[cfg(feature = "native-tls")]
pub const DEF_SSL_PORT: u16 = 443;
pub const DEF_ACCEPT: &'static str = "*/*";
pub const DEF_CONN: &'static str = "close";

pub const H_HOST: &'static str = "Host";
pub const H_ACCPT: &'static str = "Accept";
pub const H_CONN: &'static str = "Connection";
pub const H_CTYPE: &'static str = "Content-Type";
pub const H_CLEN: &'static str = "Content-Length";
