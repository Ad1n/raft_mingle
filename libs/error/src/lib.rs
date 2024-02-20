use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type SimpleResult<T> = std::result::Result<T, GenericError>;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(str: &str) -> Self {
        Self {
            message: String::from(str),
        }
    }
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Custom Error: {}", self.message))
    }
}
