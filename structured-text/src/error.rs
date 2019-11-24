use crate::parser::Rule;
use pest::{
    error::{Error as PestError, ErrorVariant},
    iterators::Pair,
};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    inner: PestError<Rule>,
}

impl ParseError {
    pub(crate) fn expect_rule(
        rule: Rule,
        pair: &Pair<'_, Rule>,
    ) -> Result<(), ParseError> {
        if pair.as_rule() == rule {
            Ok(())
        } else {
            Err(ParseError {
                inner: PestError::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![rule],
                        negatives: vec![pair.as_rule()],
                    },
                    pair.as_span(),
                ),
            })
        }
    }
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
