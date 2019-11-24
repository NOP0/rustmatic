use crate::{
    parser::{RawParser, Rule},
    ParseError,
};
use codespan::Span;
use pest::{
    error::{Error as PestError, ErrorVariant as PestErrorVariant},
    iterators::Pair,
    Parser,
};
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
        match pair.as_rule() {
            Rule::identifier => {
                Ok(Expression::Variable(Identifier::from_pair(pair)?))
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
        ParseError::expect_rule(Rule::assign, &items.next().unwrap())?;
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
        ParseError::expect_rule(Rule::infix, &pair)?;

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
        match pair.as_rule() {
            Rule::plus => Ok(BinaryOp::Add),
            Rule::minus => Ok(BinaryOp::Subtract),
            Rule::multiply => Ok(BinaryOp::Multiply),
            Rule::divide => Ok(BinaryOp::Divide),
            _ => Err(ParseError::custom(
                "Unknown binary operator",
                pair.as_span(),
            )),
        }
    }
}

pub enum Literal {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Boolean(BooleanLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: u64,
    pub span: Span,
}

impl IntegerLiteral {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<IntegerLiteral, ParseError> {
        let span = to_span(pair.as_span());

        if pair.as_rule() == Rule::integer {
            // we need to re-wrap the IntegerLiteral so the span includes
            // the leading 0x or 0b sigil
            let lit =
                IntegerLiteral::from_pair(pair.into_inner().next().unwrap())?;

            return Ok(IntegerLiteral {
                value: lit.value,
                span,
            });
        }

        let radix = match pair.as_rule() {
            Rule::integer_decimal => 10,
            Rule::integer_hexadecimal => 16,
            Rule::integer_binary => 2,
            _ => {
                return Err(PestError::new_from_span(
                    PestErrorVariant::ParsingError {
                        positives: vec![
                            Rule::integer_decimal,
                            Rule::integer_hexadecimal,
                            Rule::integer_binary,
                        ],
                        negatives: vec![],
                    },
                    pair.as_span(),
                )
                .into());
            },
        };

        Ok(IntegerLiteral {
            value: u64::from_str_radix(pair.as_str(), radix).unwrap(),
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteral {
    pub value: f64,
    pub span: Span,
}

impl FloatLiteral {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<FloatLiteral, ParseError> {
        ParseError::expect_rule(Rule::float, &pair)?;

        Ok(FloatLiteral {
            value: pair.as_str().parse().unwrap(),
            span: to_span(pair.as_span()),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub span: Span,
}

impl BooleanLiteral {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<BooleanLiteral, ParseError> {
        ParseError::expect_rule(Rule::boolean, &pair)?;

        let span = pair.as_span();
        let inner = pair.into_inner().next().unwrap();

        let value = match inner.as_rule() {
            Rule::boolean_true => true,
            Rule::boolean_false => false,
            _ => {
                return Err(PestError::new_from_span(
                    PestErrorVariant::ParsingError {
                        positives: vec![
                            Rule::boolean_true,
                            Rule::boolean_false,
                        ],
                        negatives: vec![],
                    },
                    span,
                )
                .into())
            },
        };

        Ok(BooleanLiteral {
            value,
            span: to_span(span),
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
    BinaryOp => binary_operator,
    BinaryExpression => infix,
    FloatLiteral => float,
    IntegerLiteral => integer,
    BooleanLiteral => boolean,
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
                    assign(2, 4),
                    identifier(5, 6),
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
            rule: Rule::infix,
            tokens: [
                infix(0, 5, [
                    identifier(0, 1),
                    plus(2, 3),
                    identifier(4, 5),
                ])
            ]
        }

        let got = BinaryExpression::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_false() {
        let src = "FALSE";
        let expected = BooleanLiteral {
            value: false,
            span: Span::new(0, 5),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::boolean,
            tokens: [
                boolean(0, 5, [boolean_false(0, 5)]),
            ]
        }

        let got = BooleanLiteral::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_true() {
        let src = "TRue";
        let expected = BooleanLiteral {
            value: true,
            span: Span::new(0, 4),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::boolean,
            tokens: [
                boolean(0, 4, [boolean_true(0, 4)]),
            ]
        }

        let got = BooleanLiteral::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_basic_float() {
        let src = "3.14";
        let expected = FloatLiteral {
            value: 3.14,
            span: Span::new(0, 4),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::float,
            tokens: [
                float(0, 4, [float_characteristic(0, 1), float_mantissa(2, 4)]),
            ]
        }

        let got = FloatLiteral::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_an_integer() {
        let src = "42";
        let expected = IntegerLiteral {
            value: 42,
            span: Span::new(0, 2),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::integer,
            tokens: [
                integer(0, 2, [integer_decimal(0, 2)]),
            ]
        }

        let got = IntegerLiteral::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_a_hex_integer() {
        let src = "0xdeadbeef";
        let expected = IntegerLiteral {
            value: 0xdeadbeef,
            span: Span::new(0, 10),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::integer,
            tokens: [
                integer(0, 10, [integer_hexadecimal(2, 10)]),
            ]
        }

        let got = IntegerLiteral::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
