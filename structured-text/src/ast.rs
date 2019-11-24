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
        ParseError::expect_rule(Rule::identifier, &pair)?;

        Ok(Identifier {
            value: pair.as_str().to_string(),
            span: to_span(pair.as_span()),
        })
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
        ParseError::expect_rule(Rule::variable_decl, &pair)?;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(Identifier),
}

impl Expression {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Expression, ParseError> {
        ParseError::expect_rule(Rule::expression, &pair)?;

        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::identifier => {
                Ok(Expression::Variable(Identifier::from_pair(inner)?))
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub variable: Identifier,
    pub value: Expression,
    pub span: Span,
}

impl Assignment {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Assignment, ParseError> {
        ParseError::expect_rule(Rule::assignment, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let variable = Identifier::from_pair(items.next().unwrap())?;
        let value = Expression::from_pair(items.next().unwrap())?;

        Ok(Assignment {
            variable,
            value,
            span,
        })
    }
}

macro_rules! impl_from_str {
    ($( $name:ty => $rule:ident, )*) => {
        $(
            impl FromStr for $name {
                type Err = ParseError;

                fn from_str(src: &str) -> Result<$name, ParseError> {
                    let mut pairs = RawParser::parse(Rule::$rule, src)?;

                    <$name>::from_pair(pairs.next().unwrap())
                }
            }
        )*
    };
}

impl_from_str! {
    Identifier => identifier,
    Assignment => assignment,
    VariableDeclaration => variable_decl,
    Expression => expression,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        let src = "x_32";
        let expected = Identifier {
            value: String::from("x_32"),
            span: Span::new(0, 4),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::identifier,
            tokens: [
                    identifier(0, 4)
            ]
        }

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

    #[test]
    fn simple_assignment() {
        let src = "x := y";
        let expected = Assignment {
            variable: Identifier {
                value: String::from("x"),
                span: Span::new(0, 1),
            },
            value: Expression::Variable(Identifier {
                value: String::from("y"),
                span: Span::new(5, 6),
            }),
            span: Span::new(0, 6),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::assignment,
            tokens: [
                assignment(0, 6, [
                    identifier(0, 1),
                    expression(5, 6, [
                        identifier(5, 6),
                    ])
                ])
            ]
        }

        let got = Assignment::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
