pub mod http_status;

pub use super::http_version::HttpVersion;
use http_status::HttpStatus;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatus,
}
