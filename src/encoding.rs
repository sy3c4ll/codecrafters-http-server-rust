use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Gzip,
}

impl Encoding {
    pub fn as_str(&self) -> &'static str {
        match self {
            Encoding::Gzip => "gzip",
        }
    }

    pub fn from_header(header: impl AsRef<str>) -> Vec<Option<Self>> {
        header
            .as_ref()
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .map(Result::ok)
            .collect()
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            _ => Err(()),
        }
    }
}
