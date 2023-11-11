use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HttpVersion {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2,
    Http3,
}

impl HttpVersion {
    pub const fn from_bytes(name: &[u8]) -> Option<Self> {
        match name {
            b"HTTP/0.9" => Some(HttpVersion::Http0_9),
            b"HTTP/1.0" => Some(HttpVersion::Http1_0),
            b"HTTP/1.1" => Some(HttpVersion::Http1_1),
            b"HTTP/2" => Some(HttpVersion::Http2),
            b"HTTP/3" => Some(HttpVersion::Http3),
            _ => None,
        }
    }
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::Http0_9 => "HTTP/0.9",
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}