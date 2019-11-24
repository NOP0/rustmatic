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

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub name: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Block,
    pub span: Span,
}

impl Program {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Program, ParseError> {
        ParseError::expect_rule(Rule::program, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();

        let name = Identifier::from_pair(items.next().unwrap())?;

        let var_blocks = items
            .next()
            .unwrap()
            .into_inner()
            .map(VarBlock::from_pair)
            .collect::<Result<Vec<_>, _>>()?;

        let body = Block::from_pair(items.next().unwrap())?;

        Ok(Program {
            name,
            var_blocks,
            body,
            span,
        })
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct VarBlock {
    pub declarations: Vec<VariableDeclaration>,
    pub kind: VarBlockKind,
    pub span: Span,
}

impl VarBlock {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<VarBlock, ParseError> {
        ParseError::expect_rule(Rule::var_block, &pair)?;

        let span = to_span(pair.as_span());
        let mut items = pair.into_inner();

        let kind = VarBlockKind::from_pair(items.next().unwrap())?;

        let declarations = items
            .map(VariableDeclaration::from_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(VarBlock {
            declarations,
            kind,
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarBlockKind {
    Normal,
    Global,
    External,
    Input,
    Output,
}

impl VarBlockKind {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<VarBlockKind, ParseError> {
        match pair.as_rule() {
            Rule::normal_var_block => Ok(VarBlockKind::Normal),
            Rule::global_var_block => Ok(VarBlockKind::Global),
            Rule::external_var_block => Ok(VarBlockKind::External),
            Rule::input_var_block => Ok(VarBlockKind::Input),
            Rule::output_var_block => Ok(VarBlockKind::Output),
            _ => Err(ParseError::expected_one_of(
                &[
                    Rule::var_block,
                    Rule::global_var_block,
                    Rule::external_var_block,
                    Rule::input_var_block,
                    Rule::output_var_block,
                ],
                pair.as_span(),
            )),
        }
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
    Literal(Literal),
    BinaryExpression(BinaryExpression),
}

impl Expression {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Expression, ParseError> {
        match pair.as_rule() {
            Rule::infix => {
                Expression::from_pair(pair.into_inner().next().unwrap())
            },
            Rule::identifier => {
                Ok(Expression::Variable(Identifier::from_pair(pair)?))
            },
            Rule::boolean | Rule::float | Rule::integer | Rule::string => {
                Ok(Expression::Literal(Literal::from_pair(pair)?))
            },
            other => unimplemented!("{:#?}", other),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Repeat {
    pub block: Block,
    pub condition: Assignment,
    pub span: Span,
}

impl Repeat {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Repeat, ParseError> {
        ParseError::expect_rule(Rule::repeat, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let block = Block::from_pair(items.next().unwrap())?;
        let condition = Assignment::from_pair(items.next().unwrap())?;

        Ok(Repeat {
            block,
            condition,
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Block {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Block, ParseError> {
        ParseError::expect_rule(Rule::block, &pair)?;

        let span = to_span(pair.as_span());
        let statements = pair
            .into_inner()
            .map(Statement::from_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Block { statements, span })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
}

impl Statement {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Statement, ParseError> {
        if pair.as_rule() == Rule::statement {}

        match pair.as_rule() {
            Rule::statement => {
                Statement::from_pair(pair.into_inner().next().unwrap())
            },
            Rule::assignment => {
                Ok(Statement::Assignment(Assignment::from_pair(pair)?))
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

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Boolean(BooleanLiteral),
}

impl Literal {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Literal, ParseError> {
        match pair.as_rule() {
            Rule::boolean => {
                Ok(Literal::Boolean(BooleanLiteral::from_pair(pair)?))
            },
            Rule::float => Ok(Literal::Float(FloatLiteral::from_pair(pair)?)),
            Rule::integer => {
                Ok(Literal::Integer(IntegerLiteral::from_pair(pair)?))
            },
            Rule::string => {
                Ok(Literal::String(StringLiteral::from_pair(pair)?))
            },
            _ => Err(ParseError::expected_one_of(
                &[Rule::boolean, Rule::float, Rule::integer, Rule::string],
                pair.as_span(),
            )),
        }
    }
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
                return Err(ParseError::expected_one_of(
                    &[
                        Rule::integer_decimal,
                        Rule::integer_hexadecimal,
                        Rule::integer_binary,
                    ],
                    pair.as_span(),
                ));
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

impl StringLiteral {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<StringLiteral, ParseError> {
        ParseError::expect_rule(Rule::string, &pair)?;

        Ok(StringLiteral {
            value: pair.as_str().to_string(),
            span: to_span(pair.as_span()),
        })
    }
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
                return Err(ParseError::expected_one_of(
                    &[Rule::boolean_true, Rule::boolean_false],
                    span,
                ));
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
    Statement => statement,
    Repeat => repeat,
    VarBlock => var_block,
    VarBlockKind => var_block_kind,
    Program => program,
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
            tokens: [float(0, 4)]
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

    #[test]
    fn simple_repeat() {
        let src = "REPEAT \nx := FALSE;\nUNTIL x := FALSE;\nEND_REPEAT;";
        let expected = Repeat {
            block: Block {
                statements: vec![Statement::Assignment(Assignment {
                    variable: Identifier {
                        value: String::from("x"),
                        span: Span::new(8, 9),
                    },
                    value: Arc::new(Expression::Literal(Literal::Boolean(
                        BooleanLiteral {
                            value: false,
                            span: Span::new(13, 18),
                        },
                    ))),
                    span: Span::new(8, 18),
                })],
                span: Span::new(8, 19),
            },
            condition: Assignment {
                variable: Identifier {
                    value: String::from("x"),
                    span: Span::new(26, 27),
                },
                value: Arc::new(Expression::Literal(Literal::Boolean(
                    BooleanLiteral {
                        value: false,
                        span: Span::new(31, 36),
                    },
                ))),
                span: Span::new(26, 36),
            },
            span: Span::new(0, 48),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::repeat,
            tokens: [
                repeat(0, 48, [
                        block(8, 19, [
                            statement(8, 19, [
                                assignment(8, 18, [
                                    identifier(8, 9),
                                    assign(10, 12),
                                    boolean(13, 18, [boolean_false(13, 18)]),
                                ])
                            ])
                        ]),
                        assignment(26, 36, [
                            identifier(26, 27),
                            assign(28, 30),
                            boolean(31, 36, [boolean_false(31, 36)]),
                        ])
                ]),
            ]
        }

        let got = Repeat::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn var_block_with_two_decls() {
        let src = "VAR\nx : BOOL;\nfourty_two : INTEGER;\nEND_VAR";
        let expected = VarBlock {
            declarations: vec![
                VariableDeclaration {
                    name: Identifier {
                        value: String::from("x"),
                        span: Span::new(4, 5),
                    },
                    declared_type: Identifier {
                        value: String::from("BOOL"),
                        span: Span::new(8, 12),
                    },
                    span: Span::new(4, 12),
                },
                VariableDeclaration {
                    name: Identifier {
                        value: String::from("fourty_two"),
                        span: Span::new(14, 24),
                    },
                    declared_type: Identifier {
                        value: String::from("INTEGER"),
                        span: Span::new(27, 34),
                    },
                    span: Span::new(14, 34),
                },
            ],
            kind: VarBlockKind::Normal,
            span: Span::new(0, 43),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::var_block,
            tokens: [
                var_block(0, 43, [
                    normal_var_block(0, 3),
                    variable_decl(4, 12, [
                        identifier(4, 5),
                        identifier(8, 12),
                    ]),
                    variable_decl(14, 34, [
                        identifier(14, 24),
                        identifier(27, 34),
                    ]),
                ]),
            ]
        }

        let got = VarBlock::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_a_program() {
        let src = r#"PROGRAM foo 
            VAR_GLOBAL 
                fourty_two: INTEGER; 
            END_VAR 
            
            fourty_two := 42; 
        END_PROGRAM
        "#;
        let expected = Program {
            name: Identifier {
                value: String::from("foo"),
                span: Span::new(8, 11),
            },
            var_blocks: vec![VarBlock {
                declarations: vec![VariableDeclaration {
                    name: Identifier {
                        value: String::from("fourty_two"),
                        span: Span::new(53, 63),
                    },
                    declared_type: Identifier {
                        value: String::from("INTEGER"),
                        span: Span::new(65, 72),
                    },
                    span: Span::new(53, 72),
                }],
                kind: VarBlockKind::Global,
                span: Span::new(25, 94),
            }],
            body: Block {
                statements: vec![Statement::Assignment(Assignment {
                    variable: Identifier {
                        value: String::from("fourty_two"),
                        span: Span::new(121, 131),
                    },
                    value: Arc::new(Expression::Literal(Literal::Integer(
                        IntegerLiteral {
                            value: 42,
                            span: Span::new(135, 137),
                        },
                    ))),
                    span: Span::new(121, 137),
                })],
                span: Span::new(121, 138),
            },
            span: Span::new(0, 168),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::program,
            tokens: [
                program(0, 168, [
                    identifier(8, 11),
                    preamble(25, 94, [
                        var_block(25, 94, [
                            global_var_block(25, 35),
                            variable_decl(53, 72, [
                                identifier(53, 63),
                                identifier(65, 72),
                            ]),
                        ]),
                    ]),
                    block(121, 138, [
                        statement(121, 138, [
                            assignment(121, 137, [
                                identifier(121, 131),
                                assign(132, 134),
                                integer(135, 137, [integer_decimal(135, 137)]),
                            ])
                        ])
                    ]),
                ]),
            ]
        }

        let got = Program::from_str(src).unwrap();

        assert_eq!(got, expected);
    }
}
