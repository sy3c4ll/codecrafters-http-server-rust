use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HttpStatus {
    code: u16,
    message: &'static str,
}

impl HttpStatus {
    pub const OK: Self = Self { code: 200, message: "OK" };
    pub const NOT_FOUND: Self = Self { code: 404, message: "Not Found" };

    pub const fn from_status_code(code: u16) -> Option<Self> {
        match code {
            200 => Some(Self::OK),
            404 => Some(Self::NOT_FOUND),
            _ => None,
        }
    }
    pub const fn code(&self) -> u16 { self.code }
    pub const fn message(&self) -> &'static str { self.message }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}