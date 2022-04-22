use std::{error::Error, fmt};
use tonic;

pub struct Status(tonic::Status);

impl Status {
    pub fn new(msg: &str, err: impl Error) -> Self {
        Self(tonic::Status::internal(format!("{}: '{}'", msg, err)))
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self(tonic::Status::internal(msg))
    }

    pub fn invalid_argument(msg: &str) -> Self {
        Self(tonic::Status::invalid_argument(msg))
    }

    pub fn not_found(msg: &str) -> Self {
        Self(tonic::Status::not_found(msg))
    }
}

impl From<std::io::Error> for Status {
    fn from(err: std::io::Error) -> Self {
        Self(tonic::Status::from(err))
    }
}

impl From<String> for Status {
    fn from(err: String) -> Self {
        Self(tonic::Status::internal(err))
    }
}

impl Error for Status {}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
