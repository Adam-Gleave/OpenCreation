use std::error;
use std::io;

#[derive(Debug, Clone)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: &str) -> ParseError {
        Self {
            description: description.to_owned(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::new(&error.to_string())
    }
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for ParseError {
    fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
        Self::new(&error.to_string())
    }
}
