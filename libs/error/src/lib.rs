use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub enum ApplicationError<T: Error> {
    Internal(InternalError<T>),
    Custom(CustomError),
}

pub struct InternalError<T>
where
    T: Error,
{
    inner: Option<T>,
}

impl<T> InternalError<T>
where
    T: Error + Display,
{
    pub fn new(src: T) -> Self {
        Self { inner: Some(src) }
    }
}

impl<T> Display for InternalError<T>
where
    T: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(error) => f.write_fmt(format_args!("Internal error:  {}", error)),
            None => f.write_str("Internal error"),
        }
    }
}

impl<T: Error> Debug for InternalError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternalError")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: Error> Error for InternalError<T> {}

impl<T> From<T> for InternalError<T>
where
    T: Error,
{
    fn from(src: T) -> InternalError<T> {
        InternalError::new(src)
    }
}

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
