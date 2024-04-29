use std::{error::Error, fmt};
use crate::constants::MAX_INDEXABLE_HOSTS;

#[derive(Debug)]
pub enum CustomError{
    Connection,
    Timeout,
    TooManyHosts
}

impl Error for CustomError{}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Connection => write!(f, "Connection error occurred"),
            CustomError::Timeout => write!(f, "Timeout error occurred"),
            CustomError::TooManyHosts => write!(f, "Maximum host limit of {} surpassed", MAX_INDEXABLE_HOSTS)
        }
    }
}