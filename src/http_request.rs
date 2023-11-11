pub mod http_method;

pub use super::http_version::HttpVersion;
use http_method::HttpMethod;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: PathBuf,
    pub version: HttpVersion,
    pub headers: HashMap<Vec<u8>, Vec<u8>>,
}
