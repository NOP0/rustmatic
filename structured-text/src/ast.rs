use crate::{
    parser::{Parser, Rule},
    ParseError,
};
use codespan::Span;
use pest::{iterators::Pair, Parser as _};
use std::str::FromStr;

fn to_span(pest_span: pest::Span<'_>) -> Span {
    Span::new(pest_span.start() as u32, pest_span.end() as u32)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: String,
    pub span: Span,
}

impl Identifier {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Identifier, ParseError> {
        match pair.as_rule() {
            Rule::identifier => Ok(Identifier {
                value: pair.as_str().to_string(),
                span: to_span(pair.as_span()),
            }),
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Identifier, ParseError> {
        let mut pairs = Parser::parse(Rule::identifier, src)?;

        Identifier::from_pair(pairs.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_identifier() {
        let src = "x";
        let expected = Identifier {
            value: String::from("x"),
            span: Span::new(0, 1),
        };

        let got = Identifier::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
