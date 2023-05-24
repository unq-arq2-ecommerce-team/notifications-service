use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

#[derive(Debug, PartialEq, ToSchema)]
pub enum ErrorKind {
    BadInput,
    Other,
}

pub fn msg(err: impl fmt::Display) -> Error {
    Error {
        kind: ErrorKind::Other,
        message: err.to_string().into(),
    }
}

pub fn bad_input(msg: impl fmt::Display) -> Error {
    Error {
        kind: ErrorKind::BadInput,
        message: msg.to_string().into(),
    }
}

impl Error {
    pub(crate) fn split(self) -> (ErrorKind, String) {
        (self.kind, self.message)
    }
}