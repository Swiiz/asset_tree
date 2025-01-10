use std::{fmt::Debug, path::PathBuf};

pub struct Error<T: std::error::Error> {
    parent: PathBuf,
    kind: ErrorKind<T>,
}

pub enum ErrorKind<T> {
    Loader(T),
    Deserialization(Box<dyn std::error::Error>),
}

impl<T: std::error::Error> Error<T> {
    pub fn deserialization(parent: PathBuf, error: Box<dyn std::error::Error>) -> Error<T> {
        Error {
            parent,
            kind: ErrorKind::Deserialization(error),
        }
    }

    pub fn loader(parent: PathBuf, error: T) -> Error<T> {
        Error {
            parent,
            kind: ErrorKind::Loader(error),
        }
    }
}

impl<T: std::error::Error> std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Loader(e) => {
                write!(f, "Error loading asset at {}: {}", self.parent.display(), e)
            }
            ErrorKind::Deserialization(e) => {
                write!(
                    f,
                    "Error deserializing asset at {}: {}",
                    self.parent.display(),
                    e
                )
            }
        }
    }
}

impl<T: std::error::Error> Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl<T: std::error::Error> std::error::Error for Error<T> {}
