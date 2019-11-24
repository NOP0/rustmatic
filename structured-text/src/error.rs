use crate::parser::Rule;
use pest::error::Error as PestError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    inner: PestError<Rule>,
}

// this is just an implementation detail to make `?` more useful
#[doc(hidden)]
impl From<PestError<Rule>> for ParseError {
    fn from(other: PestError<Rule>) -> ParseError {
        ParseError { inner: other }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.inner) }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse the input text")
    }
}
