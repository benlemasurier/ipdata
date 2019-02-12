use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IpDataError {
    message: String,
}

impl IpDataError {
    pub fn new(message: &str) -> IpDataError {
        IpDataError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for IpDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IpDataError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<reqwest::UrlError> for IpDataError {
    fn from(err: reqwest::UrlError) -> Self {
        IpDataError::new(err.description())
    }
}
