pub mod http_status;

pub use super::http_version::HttpVersion;
use http_status::HttpStatus;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}