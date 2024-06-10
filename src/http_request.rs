pub mod http_method;

use super::http_version::HttpVersion;
use http_method::HttpMethod;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: PathBuf,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse_request(buf: &[u8]) -> Option<Self> {
        let mut lines = buf
            .split(|c| *c == b'\n')
            .map(|s| s.strip_suffix(b"\r").unwrap_or(s))
            .skip_while(|s| s.is_empty());

        let start = std::str::from_utf8(lines.next()?).ok()?;
        let mut start = start
            .split_whitespace()
            .filter(|s| !s.is_empty());
        let (Some(method), Some(path), Some(version), None) =
            (start.next(), start.next(), start.next(), start.next()) else {
            return None;
        };
        let (method, path, version) =
            (str::parse(method).ok()?, PathBuf::from(path), str::parse(version).ok()?);

        let headers = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|line| std::str::from_utf8(line)
                .map_or(None, |s| s.split_once(':') )
                .map(|(k, v)| (k.to_lowercase(), v.trim().to_owned())))
            .collect::<Option<HashMap<_, _>>>()?;

        let body = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .collect::<Vec<&[u8]>>()
            .join::<&[u8]>(b"\r\n");

        Some(Self { method, path, version, headers, body })
    }
}
