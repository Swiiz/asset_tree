use core::fmt::Debug;

use alloc::{boxed::Box, string::String};

pub struct Error<T: core::error::Error + Send + Sync + 'static> {
    parent: String,
    pub kind: ErrorKind<T>,
}

pub enum ErrorKind<T> {
    Loader(T),
    Deserialization(Box<dyn core::error::Error + Send + Sync>),
}

impl<T: core::error::Error + Send + Sync + 'static> Error<T> {
    pub fn deserialization(
        parent: String,
        error: Box<dyn core::error::Error + Send + Sync>,
    ) -> Error<T> {
        Error {
            parent,
            kind: ErrorKind::Deserialization(error),
        }
    }

    pub fn loader(parent: String, error: T) -> Error<T> {
        Error {
            parent,
            kind: ErrorKind::Loader(error),
        }
    }
}

impl<T: core::error::Error + Send + Sync + 'static> core::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            ErrorKind::Loader(e) => {
                write!(f, "Error loading asset at {}: {}", self.parent, e)
            }
            ErrorKind::Deserialization(e) => {
                write!(f, "Error deserializing asset at {}: {}", self.parent, e)
            }
        }
    }
}

impl<T: core::error::Error + Send + Sync + 'static> Debug for Error<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<T: core::error::Error + Send + Sync + 'static> core::error::Error for Error<T> {}
