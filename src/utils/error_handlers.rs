use std::fmt;

use deadpool_diesel::InteractError;

#[derive(Debug)]
pub enum MappedErrors {
    InternalServerError,
    NotFound,
}

pub fn error_mapper<T: Error>(error: T) -> MappedErrors {
    error.as_infra_error()
}

impl fmt::Display for MappedErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MappedErrors::NotFound => write!(f, "Not found"),
            MappedErrors::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

pub trait Error {
    fn as_infra_error(&self) -> MappedErrors;
}

impl Error for diesel::result::Error {
    fn as_infra_error(&self) -> MappedErrors {
        match self {
            diesel::result::Error::NotFound => MappedErrors::NotFound,
            _ => MappedErrors::InternalServerError,
        }
    }
}

impl Error for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> MappedErrors {
        MappedErrors::InternalServerError
    }
}

impl Error for InteractError {
    fn as_infra_error(&self) -> MappedErrors {
        MappedErrors::InternalServerError
    }
}
