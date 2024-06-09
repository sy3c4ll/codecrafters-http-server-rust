use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HttpVersion {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2,
    Http3,
}

impl HttpVersion {
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
        f.write_str(self.as_str())
    }
}

impl FromStr for HttpVersion {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "HTTP/0.9" => Ok(HttpVersion::Http0_9),
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(()),
        }
    }
}
