use std::fmt;

#[derive(Debug)]
pub enum LoadTestingError {
    FailedTransaction,
    // You can add more error types here
}

impl fmt::Display for LoadTestingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadTestingError::FailedTransaction => write!(f, "Transaction failed"),
            // Add more error types here
        }
    }
}

impl std::error::Error for LoadTestingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            LoadTestingError::FailedTransaction => None,
            // Add more error types here
        }
    }
}
