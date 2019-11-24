use crate::{
    parser::{RawParser, Rule},
    ParseError,
};
use codespan::Span;
use pest::{iterators::Pair, Parser};
use std::{str::FromStr, sync::Arc};

fn to_span(pest_span: pest::Span<'_>) -> Span {
    Span::new(pest_span.start() as u32, pest_span.end() as u32)
}

/// An identifier, typically used when naming variables or functions.
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

/// A single variable declaration (e.g. `x: BOOL`).
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

/// An expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(Identifier),
    BinaryExpression(BinaryExpression),
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

/// An assignment statement (e.g. `x := 42`).
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub variable: Identifier,
    pub value: Arc<Expression>,
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
            span,
            value: Arc::new(value),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Arc<Expression>,
    pub right: Arc<Expression>,
    pub op: BinaryOp,
    pub span: Span,
}

impl BinaryExpression {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<BinaryExpression, ParseError> {
        ParseError::expect_rule(Rule::binary_expression, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let left = Expression::from_pair(items.next().unwrap())?;
        let op = BinaryOp::from_pair(items.next().unwrap())?;
        let right = Expression::from_pair(items.next().unwrap())?;

        Ok(BinaryExpression {
            left: Arc::new(left),
            right: Arc::new(right),
            op,
            span,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOp {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<BinaryOp, ParseError> {
        ParseError::expect_rule(Rule::bin_op, &pair)?;

        match pair.as_str() {
            "+" => Ok(BinaryOp::Add),
            _ => unimplemented!(),
        }
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
    BinaryExpression => binary_expression,
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
            value: Arc::new(Expression::Variable(Identifier {
                value: String::from("y"),
                span: Span::new(5, 6),
            })),
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

    #[test]
    fn a_plus_b() {
        let src = "a + b";
        let expected = BinaryExpression {
            left: Arc::new(Expression::Variable(Identifier {
                value: String::from("a"),
                span: Span::new(0, 1),
            })),
            right: Arc::new(Expression::Variable(Identifier {
                value: String::from("b"),
                span: Span::new(4, 5),
            })),
            op: BinaryOp::Add,
            span: Span::new(0, 5),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::binary_expression,
            tokens: [
                binary_expression(0, 5, [
                    expression(0, 1, [ identifier(0, 1) ]),
                    bin_op(2, 3),
                    expression(4, 5, [ identifier(4, 5) ]),
                ])
            ]
        }

        let got = BinaryExpression::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
