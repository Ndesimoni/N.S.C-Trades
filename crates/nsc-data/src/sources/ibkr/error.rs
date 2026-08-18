use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IbkrError {
    Connection(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for IbkrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IbkrError::Connection(err) => {
                write!(f, "IBKR connection failed: {err}")
            }
        }
    }
}

impl Error for IbkrError {}
