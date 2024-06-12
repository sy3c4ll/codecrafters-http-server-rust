pub mod http_status;

use super::encoding::Encoding;
use super::http_request::{HttpRequest, http_method::HttpMethod};
use super::http_version::HttpVersion;
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
            let encoding = match request.headers.get("accept-encoding") {
                Some(header) => header
                    .split(',')
                    .map(str::trim)
                    .map(str::parse)
                    .find_map(Result::ok)
                    .unwrap_or(Encoding::Identity),
                None => Encoding::Identity,
            };
            let data = encoding.encode(data.as_bytes());
            Self {
                version: request.version,
                status: HttpStatus::OK,
                headers: match encoding {
                    Encoding::Identity => HashMap::from([
                        ("Content-Type".to_owned(), "text/plain".to_owned()),
                        ("Content-Length".to_owned(), data.len().to_string()),
                    ]),
                    _ => HashMap::from([
                        ("Content-Encoding".to_owned(), encoding.to_string()),
                        ("Content-Type".to_owned(), "text/plain".to_owned()),
                        ("Content-Length".to_owned(), data.len().to_string()),
                    ]),
                },
                body: data.to_owned(),
            }
        } else if request.path == Path::new("/user-agent") {
            match request.headers.get("user-agent") {
                Some(data) => Self {
                    version: request.version,
                    status: HttpStatus::OK,
                    headers: HashMap::from([
                        ("Content-Type".to_owned(), "text/plain".to_owned()),
                        ("Content-Length".to_owned(), data.as_bytes().len().to_string()),
                    ]),
                    body: data.as_bytes().to_owned(),
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
                .and_then(|i| std::env::args().nth(i + 1))
                .map(|s| Path::new(&s).join(data))
                .and_then(|p| std::fs::read(p).ok());
            match body {
                Some(body) => Self {
                    version: request.version,
                    status: HttpStatus::OK,
                    headers: HashMap::from([
                        ("Content-Type".to_owned(), "application/octet-stream".to_owned()),
                        ("Content-Length".to_owned(), body.len().to_string()),
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
                .and_then(|i| std::env::args().nth(i + 1))
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
