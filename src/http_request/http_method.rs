use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl HttpMethod {
    pub const fn from_bytes(name: &[u8]) -> Option<Self> {
        match name {
            b"GET" => Some(HttpMethod::Get),
            b"HEAD" => Some(HttpMethod::Head),
            b"POST" => Some(HttpMethod::Post),
            b"PUT" => Some(HttpMethod::Put),
            b"DELETE" => Some(HttpMethod::Delete),
            b"CONNECT" => Some(HttpMethod::Connect),
            b"OPTIONS" => Some(HttpMethod::Options),
            b"TRACE" => Some(HttpMethod::Trace),
            b"PATCH" => Some(HttpMethod::Patch),
            _ => None,
        }
    }
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Patch => "PATCH",
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}