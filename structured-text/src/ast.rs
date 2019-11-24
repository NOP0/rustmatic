use crate::{
    parser::{RawParser, Rule},
    ParseError,
};
use codespan::Span;
use pest::{iterators::Pair, Parser};
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
        if pair.as_rule() != Rule::identifier {
            unimplemented!()
        }
        Ok(Identifier {
            value: pair.as_str().to_string(),
            span: to_span(pair.as_span()),
        })
    }
}

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Identifier, ParseError> {
        let mut pairs = RawParser::parse(Rule::identifier, src)?;

        Identifier::from_pair(pairs.next().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub declared_type: Identifier,
    pub span: Span,
}

impl VariableDeclaration {
    fn from_pair(
        pair: Pair<'_, Rule>,
    ) -> Result<VariableDeclaration, ParseError> {
        if pair.as_rule() != Rule::variable_decl {
            unimplemented!()
        }

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let name = Identifier::from_pair(items.next().unwrap())?;
        let declared_type = Identifier::from_pair(items.next().unwrap())?;

        Ok(VariableDeclaration {
            span,
            name,
            declared_type,
        })
    }
}

impl FromStr for VariableDeclaration {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<VariableDeclaration, ParseError> {
        let mut pairs = RawParser::parse(Rule::variable_decl, src)?;

        VariableDeclaration::from_pair(pairs.next().unwrap())
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

    #[test]
    fn parse_a_variable_declaration() {
        let src = "x : u32";
        let expected = VariableDeclaration {
            name: Identifier {
                value: String::from("x"),
                span: Span::new(0, 1),
            },
            declared_type: Identifier {
                value: String::from("u32"),
                span: Span::new(4, 7),
            },
            span: Span::new(0, 7),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::variable_decl,
            tokens: [
                variable_decl(0, 7, [
                    identifier(0, 1),
                    identifier(4, 7),
                ])
            ]
        }

        let got = VariableDeclaration::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
