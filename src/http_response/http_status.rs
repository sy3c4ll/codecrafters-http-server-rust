use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HttpStatus {
    pub code: u16,
    pub message: &'static str,
}

impl HttpStatus {
    pub const OK: Self = Self { code: 200, message: "OK" };
    pub const BAD_REQUEST: Self = Self { code: 400, message: "Bad Request" };
    pub const NOT_FOUND: Self = Self { code: 404, message: "Not Found" };
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}