pub mod http_status;

pub use super::http_request::{HttpRequest, http_method::HttpMethod};
pub use super::http_version::HttpVersion;
use http_status::HttpStatus;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn request_response(request: &HttpRequest) -> Self {
        if request.path == Path::new("/") {
            Self {
                version: request.version,
                status: HttpStatus::OK,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        } else if let Ok(Some(data)) = request.path.strip_prefix("/echo").map(|p| p.to_str()) {
            Self {
                version: request.version,
                status: HttpStatus::OK,
                headers: HashMap::from([
                    ("Content-Type".to_string(), "text/plain".to_string()),
                    ("Content-Length".to_string(), data.as_bytes().len().to_string()),
                ]),
                body: data.as_bytes().to_vec(),
            }
        } else if request.path == Path::new("/user-agent") {
            match request.headers.get(&"User-Agent".to_lowercase()) {
                Some(data) => Self {
                    version: request.version,
                    status: HttpStatus::OK,
                    headers: HashMap::from([
                        ("Content-Type".to_string(), "text/plain".to_string()),
                        ("Content-Length".to_string(), data.as_bytes().len().to_string()),
                    ]),
                    body: data.as_bytes().to_vec(),
                },
                None => Self {
                    version: request.version,
                    status: HttpStatus::NOT_FOUND,
                    headers: HashMap::new(),
                    body: Vec::new(),
                },
            }
        } else if let (HttpMethod::Get, Ok(data)) = (request.method, request.path.strip_prefix("/files")) {
            let body = std::env::args().position(|s| s == "--directory")
                .map_or(None, |i| std::env::args().nth(i + 1))
                .map(|s| Path::new(&s).join(data))
                .map_or(None, |p| std::fs::read(p).ok());
            match body {
                Some(body) => Self {
                    version: request.version,
                    status: HttpStatus::OK,
                    headers: HashMap::from([
                        ("Content-Type".to_string(), "application/octet-stream".to_string()),
                        ("Content-Length".to_string(), body.len().to_string()),
                    ]),
                    body,
                },
                None => Self {
                    version: request.version,
                    status: HttpStatus::NOT_FOUND,
                    headers: HashMap::new(),
                    body: Vec::new(),
                },
            }
        } else if let (HttpMethod::Post, Ok(data)) = (request.method, request.path.strip_prefix("/files")) {
            let path = std::env::args().position(|s| s == "--directory")
                .map_or(None, |i| std::env::args().nth(i + 1))
                .map(|s| Path::new(&s).join(data));
            match path {
                Some(path) => match std::fs::write(path, &request.body) {
                    Ok(_) => Self {
                        version: request.version,
                        status: HttpStatus::CREATED,
                        headers: HashMap::new(),
                        body: Vec::new(),
                    },
                    Err(_) => Self {
                        version: request.version,
                        status: HttpStatus::INTERNAL_SERVER_ERROR,
                        headers: HashMap::new(),
                        body: Vec::new(),
                    },
                },
                None => Self {
                    version: request.version,
                    status: HttpStatus::NOT_FOUND,
                    headers: HashMap::new(),
                    body: Vec::new(),
                },
            }
        } else {
            Self {
                version: request.version,
                status: HttpStatus::NOT_FOUND,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        }
    }

    pub fn bad_request_response() -> Self {
        Self {
            version: HttpVersion::Http1_1,
            status: HttpStatus::BAD_REQUEST,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn compose_response(&self) -> Vec<u8> {
        let header = (self.headers.iter().fold(
            format!("{} {}\r\n", self.version, self.status), 
            |acc, (key, value)| acc + format!("{}: {}\r\n", key, value).as_str()
        ) + "\r\n").into_bytes();
        let body = if self.body.is_empty() {
            Vec::new()
        } else {
            [&self.body[..], b"\r\n\r\n"].concat()
        };

        [header, body].concat()
    }
}