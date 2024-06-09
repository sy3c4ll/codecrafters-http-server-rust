use std::fmt::{self, Display};
use std::str::FromStr;

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
        f.write_str(self.as_str())
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "GET" => Ok(HttpMethod::Get),
            "HEAD" => Ok(HttpMethod::Head),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "CONNECT" => Ok(HttpMethod::Connect),
            "OPTIONS" => Ok(HttpMethod::Options),
            "TRACE" => Ok(HttpMethod::Trace),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(()),
        }
    }
}
