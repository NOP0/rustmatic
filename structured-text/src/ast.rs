use crate::{
    parser::{RawParser, Rule},
    ParseError,
};
use codespan::Span;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::str::FromStr;

#[cfg(test)]
use pretty_assertions::assert_eq;

fn to_span(pest_span: pest::Span<'_>) -> Span {
    Span::new(pest_span.start() as u32, pest_span.end() as u32)
}

fn preamble(pair: Pair<'_, Rule>) -> Result<Vec<VarBlock>, ParseError> {
    ParseError::expect_rule(Rule::preamble, &pair)?;
    pair.into_inner().map(VarBlock::from_pair).collect()
}

/// All items inside a source file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct File {
    pub functions: Vec<Function>,
    pub programs: Vec<Program>,
    pub function_blocks: Vec<FunctionBlock>,
}

impl File {
    fn from_pairs(pairs: Pairs<'_, Rule>) -> Result<File, ParseError> {
        let mut functions = Vec::new();
        let mut function_blocks = Vec::new();
        let mut programs = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    functions.push(Function::from_pair(pair)?);
                },
                Rule::function_block => {
                    function_blocks.push(FunctionBlock::from_pair(pair)?);
                },
                Rule::program => {
                    programs.push(Program::from_pair(pair)?);
                },
                Rule::EOI => {},
                _ => {
                    return Err(ParseError::expected_one_of(
                        &[Rule::function, Rule::function_block, Rule::program],
                        pair.as_span(),
                    ))
                },
            }
        }

        Ok(File {
            functions,
            function_blocks,
            programs,
        })
    }
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<File, ParseError> {
        File::from_pairs(RawParser::parse(Rule::file, src)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct Function {
    pub name: Identifier,
    pub return_type: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Block,
    pub span: Span,
}

impl Function {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Function, ParseError> {
        ParseError::expect_rule(Rule::function, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let name = Identifier::from_pair(items.next().unwrap())?;
        let return_type = Identifier::from_pair(items.next().unwrap())?;
        let var_blocks = preamble(items.next().unwrap())?;
        let body = Block::from_pair(items.next().unwrap())?;

        Ok(Function {
            name,
            return_type,
            var_blocks,
            body,
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct FunctionBlock {
    pub name: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Block,
    pub span: Span,
}

impl FunctionBlock {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<FunctionBlock, ParseError> {
        ParseError::expect_rule(Rule::function_block, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let name = Identifier::from_pair(items.next().unwrap())?;
        let var_blocks = preamble(items.next().unwrap())?;
        let body = Block::from_pair(items.next().unwrap())?;

        Ok(FunctionBlock {
            name,
            var_blocks,
            body,
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
        let var_blocks = preamble(items.next().unwrap())?;
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct Identifier {
    pub value: String,
    pub span: Span,
}

impl Identifier {
    pub fn new<S, I>(value: S, start: I, end: I) -> Self
    where
        S: Into<String>,
        I: Into<codespan::ByteIndex>,
    {
        Identifier {
            value: value.into(),
            span: Span::new(start, end),
        }
    }

    fn from_pair(pair: Pair<'_, Rule>) -> Result<Identifier, ParseError> {
        ParseError::expect_rule(Rule::identifier, &pair)?;

        Ok(Identifier {
            value: pair.as_str().to_string(),
            span: to_span(pair.as_span()),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub declared_type: Identifier,
    pub initial_value: Option<Expression>,
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

        // dont' forget to step past the assignment, if there was one
        let initial_value = match items.skip(1).next() {
            Some(pair) => Some(Expression::from_pair(pair)?),
            None => None,
        };

        Ok(VariableDeclaration {
            span,
            name,
            declared_type,
            initial_value,
        })
    }
}

/// An expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum Expression {
    Variable(Identifier),
    Literal(Literal),
    BinaryExpression(BinaryExpression),
}

impl Expression {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Expression, ParseError> {
        match pair.as_rule() {
            Rule::infix => Ok(Expression::BinaryExpression(
                BinaryExpression::from_pair(pair)?,
            )),
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum Statement {
    Assignment(Assignment),
    Repeat(Repeat),
}

impl Statement {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Statement, ParseError> {
        match pair.as_rule() {
            Rule::statement => {
                Statement::from_pair(pair.into_inner().next().unwrap())
            },
            Rule::assignment => {
                Ok(Statement::Assignment(Assignment::from_pair(pair)?))
            },
            Rule::repeat => Ok(Statement::Repeat(Repeat::from_pair(pair)?)),
            _ => Err(ParseError::expected_one_of(
                &[Rule::assignment, Rule::repeat, Rule::statement],
                pair.as_span(),
            )),
        }
    }
}

/// An assignment statement (e.g. `x := 42`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
        ParseError::expect_rule(Rule::assign, &items.next().unwrap())?;
        let value = Expression::from_pair(items.next().unwrap())?;

        Ok(Assignment {
            variable,
            span,
            value,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: BinaryOp,
    pub span: Span,
}

impl BinaryExpression {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<BinaryExpression, ParseError> {
        ParseError::expect_rule(Rule::infix, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let left = Box::new(Expression::from_pair(items.next().unwrap())?);
        let op = BinaryOp::from_pair(items.next().unwrap())?;
        let right = Box::new(Expression::from_pair(items.next().unwrap())?);

        Ok(BinaryExpression {
            left,
            right,
            op,
            span,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum BinaryOp {
    Equals,
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
            Rule::equal => Ok(BinaryOp::Equals),

            _ => Err(ParseError::expected_one_of(
                &[
                    Rule::plus,
                    Rule::minus,
                    Rule::multiply,
                    Rule::divide,
                    Rule::equal,
                ],
                pair.as_span(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct ConditionalBranch {
    pub condition: Expression,
    pub block: Block,
}

impl ConditionalBranch {
    fn from_pair(
        pair: Pair<'_, Rule>,
    ) -> Result<ConditionalBranch, ParseError> {
        let mut items = pair.into_inner();

        let condition = Expression::from_pair(items.next().unwrap())?;
        let block = Block::from_pair(items.next().unwrap())?;

        Ok(ConditionalBranch { condition, block })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-1",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct Conditional {
    pub true_branch: ConditionalBranch,
    pub else_if_branches: Vec<ConditionalBranch>,
    pub else_branch: Option<Block>,
    pub span: Span,
}

impl Conditional {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Conditional, ParseError> {
        ParseError::expect_rule(Rule::conditional, &pair)?;

        let span = to_span(pair.as_span());

        let mut items = pair.into_inner();
        let true_branch = ConditionalBranch::from_pair(items.next().unwrap())?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        while let Some(branch) = items.next() {
            match branch.as_rule() {
                Rule::elsif_branch => {
                    else_if_branches.push(ConditionalBranch::from_pair(branch)?)
                },
                Rule::else_branch => {
                    else_branch = Some(Block::from_pair(branch)?);
                    break;
                },
                _ => {
                    return Err(ParseError::expected_one_of(
                        &[Rule::elsif_branch, Rule::else_branch],
                        branch.as_span(),
                    ))
                },
            }
        }

        Ok(Conditional {
            true_branch,
            else_if_branches,
            else_branch,
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
    Assignment => assignment,
    BinaryExpression => infix,
    BinaryOp => binary_operator,
    BooleanLiteral => boolean,
    Conditional => conditional,
    Expression => expression,
    FloatLiteral => float,
    Function => function,
    FunctionBlock => function_block,
    Identifier => identifier,
    IntegerLiteral => integer,
    Program => program,
    Repeat => repeat,
    Statement => statement,
    VarBlock => var_block,
    VarBlockKind => var_block_kind,
    VariableDeclaration => variable_decl,
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
            initial_value: None,
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
    fn variable_decl_with_default_value() {
        let src = "x: INT := 42";
        let expected = VariableDeclaration {
            name: Identifier {
                value: String::from("x"),
                span: Span::new(0, 1),
            },
            declared_type: Identifier {
                value: String::from("INT"),
                span: Span::new(3, 6),
            },
            initial_value: Some(Expression::Literal(Literal::Integer(
                IntegerLiteral {
                    value: 42,
                    span: Span::new(10, 12),
                },
            ))),
            span: Span::new(0, 12),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::variable_decl,
            tokens: [
                variable_decl(0, 12, [
                    identifier(0, 1),
                    identifier(3, 6),
                    assign(7, 9),
                    integer(10, 12, [integer_decimal(10, 12)]),
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
            left: Box::new(Expression::Variable(Identifier {
                value: String::from("a"),
                span: Span::new(0, 1),
            })),
            right: Box::new(Expression::Variable(Identifier {
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
        let src = "16#deadbeef";
        let expected = IntegerLiteral {
            value: 0xdeadbeef,
            span: Span::new(0, 11),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::integer,
            tokens: [
                integer(0, 11, [integer_hexadecimal(3, 11)]),
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
                    variable: Identifier::new("x", 8, 9),
                    value: Expression::Literal(Literal::Boolean(
                        BooleanLiteral {
                            value: false,
                            span: Span::new(13, 18),
                        },
                    )),
                    span: Span::new(8, 18),
                })],
                span: Span::new(8, 19),
            },
            condition: Assignment {
                variable: Identifier::new("x", 26, 27),
                value: Expression::Literal(Literal::Boolean(BooleanLiteral {
                    value: false,
                    span: Span::new(31, 36),
                })),
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
        let src = "VAR\nx : BOOL;\nfourty_two : INT;\nEND_VAR";
        let expected = VarBlock {
            declarations: vec![
                VariableDeclaration {
                    name: Identifier::new("x", 4, 5),
                    declared_type: Identifier::new("BOOL", 8, 12),
                    initial_value: None,
                    span: Span::new(4, 12),
                },
                VariableDeclaration {
                    name: Identifier::new("fourty_two", 14, 24),
                    declared_type: Identifier::new("INT", 27, 30),
                    initial_value: None,
                    span: Span::new(14, 30),
                },
            ],
            kind: VarBlockKind::Normal,
            span: Span::new(0, 39),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::var_block,
            tokens: [
                var_block(0, 39, [
                    normal_var_block(0, 3),
                    variable_decl(4, 12, [
                        identifier(4, 5),
                        identifier(8, 12),
                    ]),
                    variable_decl(14, 30, [
                        identifier(14, 24),
                        identifier(27, 30),
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
                fourty_two: INT;
            END_VAR

            fourty_two := 42;
        END_PROGRAM
        "#;
        let expected = Program {
            name: Identifier::new("foo", 8, 11),
            var_blocks: vec![VarBlock {
                declarations: vec![VariableDeclaration {
                    name: Identifier::new("fourty_two", 51, 61),
                    declared_type: Identifier::new("INT", 63, 66),
                    initial_value: None,
                    span: Span::new(51, 66),
                }],
                kind: VarBlockKind::Global,
                span: Span::new(24, 87),
            }],
            body: Block {
                statements: vec![Statement::Assignment(Assignment {
                    variable: Identifier::new("fourty_two", 101, 111),
                    value: Expression::Literal(Literal::Integer(
                        IntegerLiteral {
                            value: 42,
                            span: Span::new(115, 117),
                        },
                    )),
                    span: Span::new(101, 117),
                })],
                span: Span::new(101, 118),
            },
            span: Span::new(0, 147),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::program,
            tokens: [
                program(0, 147, [
                    identifier(8, 11),
                    preamble(24, 87, [
                        var_block(24, 87, [
                            global_var_block(24, 34),
                            variable_decl(51, 66, [
                                identifier(51, 61),
                                identifier(63, 66),
                            ]),
                        ]),
                    ]),
                    block(101, 118, [
                        statement(101, 118, [
                            assignment(101, 117, [
                                identifier(101, 111),
                                assign(112, 114),
                                integer(115, 117, [integer_decimal(115, 117)]),
                            ])
                        ])
                    ]),
                ]),
            ]
        }

        let got = Program::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_a_function() {
        let src = r#"FUNCTION ReturnFive : INT
            VAR_INPUT
                input: INT;
            END_VAR

            ReturnFive := 5;

        END_FUNCTION
        "#;
        let expected = Function {
            name: Identifier::new("ReturnFive", 9, 19),
            return_type: Identifier::new("INT", 22, 25),
            var_blocks: vec![VarBlock {
                declarations: vec![VariableDeclaration {
                    name: Identifier::new("input", 64, 69),
                    declared_type: Identifier::new("INT", 71, 74),
                    initial_value: None,
                    span: Span::new(64, 74),
                }],
                kind: VarBlockKind::Input,
                span: Span::new(38, 95),
            }],
            body: Block {
                statements: vec![Statement::Assignment(Assignment {
                    variable: Identifier::new("ReturnFive", 109, 119),
                    value: Expression::Literal(Literal::Integer(
                        IntegerLiteral {
                            value: 5,
                            span: Span::new(123, 124),
                        },
                    )),
                    span: Span::new(109, 124),
                })],
                span: Span::new(109, 125),
            },
            span: Span::new(0, 156),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::function,
            tokens: [
                function(0, 156, [
                    identifier(9, 19),
                    identifier(22, 25),
                    preamble(38, 95, [
                        var_block(38, 95, [
                            input_var_block(38, 47),
                            variable_decl(64, 74, [
                                identifier(64, 69),
                                identifier(71, 74),
                            ]),
                        ]),
                    ]),
                    block(109, 125, [
                        statement(109, 125, [
                            assignment(109, 124, [
                                identifier(109, 119),
                                assign(120, 122),
                                integer(123, 124, [
                                    integer_decimal(123, 124),
                                ]),
                            ]),
                        ]),
                    ]),
                ])
            ]
        }

        let got = Function::from_str(src).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_a_function_block() {
        let src = r#"FUNCTION_BLOCK FB_Timed_Counter
        //=======================================================================
        // Function Block Timed Counter :  Incremental count of the timed interval
        //=======================================================================
            VAR_INPUT
                Execute         : BOOL := FALSE;        // Trigger signal to begin Timed Counting
                Time_Increment  : REAL := 1.25;         // Enter Cycle Time (Seconds) between counts
            END_VAR

            VAR
                CycleTimer      : TON;                  // Timer FB from Command Library
                CycleCounter    : CTU;                  // Counter FB from Command Library
                TimerPreset     : TIME;                 // Converted Time_Increment in Seconds to MS
            END_VAR

            Current_Count := CycleCounter;
            Count_Complete := CycleCounter;
        END_FUNCTION_BLOCK"#;
        let expected = FunctionBlock {
            name: Identifier::new("FB_Timed_Counter", 15, 31),
            var_blocks: vec![
                VarBlock {
                    declarations: vec![
                        VariableDeclaration {
                            name: Identifier::new("Execute", 317, 324),
                            declared_type: Identifier::new("BOOL", 335, 339),
                            initial_value: Some(Expression::Literal(
                                Literal::Boolean(BooleanLiteral {
                                    value: false,
                                    span: Span::new(343, 348),
                                }),
                            )),
                            span: Span::new(317, 348),
                        },
                        VariableDeclaration {
                            name: Identifier::new("Time_Increment", 415, 429),
                            declared_type: Identifier::new("REAL", 433, 437),
                            initial_value: Some(Expression::Literal(
                                Literal::Float(FloatLiteral {
                                    value: 1.25,
                                    span: Span::new(441, 445),
                                }),
                            )),
                            span: Span::new(415, 445),
                        },
                    ],
                    kind: VarBlockKind::Input,
                    span: Span::new(291, 519),
                },
                VarBlock {
                    declarations: vec![
                        VariableDeclaration {
                            name: Identifier::new("CycleTimer", 553, 563),
                            declared_type: Identifier::new("TON", 571, 574),
                            initial_value: None,
                            span: Span::new(553, 574),
                        },
                        VariableDeclaration {
                            name: Identifier::new("CycleCounter", 642, 654),
                            declared_type: Identifier::new("CTU", 660, 663),
                            initial_value: None,
                            span: Span::new(642, 663),
                        },
                        VariableDeclaration {
                            name: Identifier::new("TimerPreset", 733, 744),
                            declared_type: Identifier::new("TIME", 751, 755),
                            initial_value: None,
                            span: Span::new(733, 755),
                        },
                    ],
                    kind: VarBlockKind::Normal,
                    span: Span::new(533, 837),
                },
            ],
            body: Block {
                statements: vec![
                    Statement::Assignment(Assignment {
                        variable: Identifier::new("Current_Count", 851, 864),
                        value: Expression::Variable(Identifier::new(
                            "CycleCounter",
                            868,
                            880,
                        )),
                        span: Span::new(851, 880),
                    }),
                    Statement::Assignment(Assignment {
                        variable: Identifier::new("Count_Complete", 894, 908),
                        value: Expression::Variable(Identifier::new(
                            "CycleCounter",
                            912,
                            924,
                        )),
                        span: Span::new(894, 924),
                    }),
                ],
                span: Span::new(851, 925),
            },
            span: Span::new(0, 952),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::function_block,
            tokens: [
              function_block(0, 952, [
                identifier(15, 31),
                preamble(291, 837, [
                  var_block(291, 519, [
                    input_var_block(291, 300),
                    variable_decl(317, 348, [
                      identifier(317, 324),
                      identifier(335, 339),
                      assign(340, 342),
                      boolean(343, 348, [
                        boolean_false(343, 348),
                      ]),
                    ]),
                    variable_decl(415, 445, [
                      identifier(415, 429),
                      identifier(433, 437),
                      assign(438, 440),
                      float(441, 445),
                    ]),
                  ]),
                  var_block(533, 837, [
                    normal_var_block(533, 536),
                    variable_decl(553, 574, [
                      identifier(553, 563),
                      identifier(571, 574),
                    ]),
                    variable_decl(642, 663, [
                      identifier(642, 654),
                      identifier(660, 663),
                    ]),
                    variable_decl(733, 755, [
                      identifier(733, 744),
                      identifier(751, 755),
                    ]),
                  ]),
                ]),
                block(851, 925, [
                  statement(851, 881, [
                    assignment(851, 880, [
                      identifier(851, 864),
                      assign(865, 867),
                      identifier(868, 880),
                    ]),
                  ]),
                  statement(894, 925, [
                    assignment(894, 924, [
                      identifier(894, 908),
                      assign(909, 911),
                      identifier(912, 924),
                    ]),
                  ]),
                ]),
              ]),
            ]
        }

        let got = FunctionBlock::from_str(src).unwrap();

        assert_eq!(got, expected);

        // ... my god that was a pain to type out
    }

    #[test]
    fn solitary_if() {
        let src = "if x = true THEN\nx := false;\nend_if";
        let expected = Conditional {
            true_branch: ConditionalBranch {
                condition: Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(Expression::Variable(Identifier::new(
                        "x", 3, 4,
                    ))),
                    right: Box::new(Expression::Literal(Literal::Boolean(
                        BooleanLiteral {
                            value: true,
                            span: Span::new(7, 11),
                        },
                    ))),
                    op: BinaryOp::Equals,
                    span: Span::new(3, 12),
                }),
                block: Block {
                    statements: vec![Statement::Assignment(Assignment {
                        variable: Identifier::new("x", 17, 18),
                        value: Expression::Literal(Literal::Boolean(
                            BooleanLiteral {
                                value: false,
                                span: Span::new(22, 27),
                            },
                        )),
                        span: Span::new(17, 27),
                    })],
                    span: Span::new(17, 28),
                },
            },
            else_if_branches: vec![],
            else_branch: None,
            span: Span::new(0, 35),
        };

        parses_to! {
            parser: RawParser,
            input: src,
            rule: Rule::conditional,
            tokens: [
                conditional(0, 35, [
                  if_branch(0, 28, [
                    infix(3, 12, [
                      identifier(3, 4),
                      equal(5, 6),
                      boolean(7, 11, [
                        boolean_true(7, 11),
                      ]),
                    ]),
                    block(17, 28, [
                      statement(17, 28, [
                        assignment(17, 27, [
                          identifier(17, 18),
                          assign(19, 21),
                          boolean(22, 27, [
                            boolean_false(22, 27),
                          ]),
                        ]),
                      ]),
                    ]),
                  ]),
                ]),
            ]
        }

        let got = Conditional::from_str(src).unwrap();
        println!("{:#?}", got);

        assert_eq!(got, expected);
    }

    /// A way to cheat [`parses_to!()`] when you want to see what a parse tree
    /// would look like.
    fn _pretty_print(pair: Pair<'_, Rule>, indent_level: usize) {
        let span = pair.as_span();

        let indent = "  ".repeat(indent_level);
        print!(
            "{}{:?}({}, {}",
            indent,
            pair.as_rule(),
            span.start(),
            span.end()
        );

        let children: Vec<_> = pair.into_inner().collect();

        if children.is_empty() {
            println!("),");
        } else {
            println!(", [");

            for child in children {
                _pretty_print(child, indent_level + 1);
            }
            println!("{}]),", indent);
        }
    }
}
