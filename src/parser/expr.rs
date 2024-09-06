use chumsky::{
    prelude::{choice, recursive},
    Parser,
};

use crate::parser::{
    ident::Ident,
    literals::{NumLit, StringLit},
};

use super::{
    bin_ops::{BinExpr, Precedence},
    parsable::{Parsable, ParserError},
    statement::Statement,
    syntax_elements::{DelimiterOp, LCurly, LParen, RCurly, RParen},
};

/// Block of Code. Used in if's, matches, fn bodies, ...
#[derive(Debug, PartialEq)]
pub struct CodeScope {
    pub statements: Vec<Statement>,
    pub expr: Expression,
}

impl Parsable for CodeScope {
    fn parser() -> impl Parser<char, Self, Error = ParserError>
    where
        Self: Sized,
    {
        LCurly::parser()
            .ignore_then(Statement::parser().repeated().collect())
            .then(Expression::parser())
            .then_ignore(RCurly::parser())
            .map(|(statements, expr)| Self { statements, expr })
    }
}

#[test]
fn test_scope() {
    use crate::parser::statement::Let;
    let parse = |str| CodeScope::parser().parse(str);

    assert_eq!(
        parse("{ 1 }").unwrap(),
        CodeScope {
            statements: Vec::new(),
            expr: Expression::NumLit(NumLit(1_f64))
        }
    );

    assert_eq!(
        parse(
            "{\
              let mut test = \"Statement\";
             \"TestStatement\"\
             }"
        )
        .unwrap(),
        CodeScope {
            statements: vec![Statement::Let(Let {
                is_mut: true,

                left: Ident::from_str("test").into(),
                right: Expression::StringLit(StringLit("Statement".to_owned()))
            })],
            expr: Expression::StringLit(StringLit("TestStatement".to_owned()))
        }
    );
}

#[derive(Debug, PartialEq)]
pub struct FnCall {
    fn_name: Ident,
    args: Vec<Expression>,
}

impl FnCall {
    fn parser_with(
        existing_parser: impl Parser<char, Expression, Error = ParserError>,
    ) -> impl Parser<char, Self, Error = super::parsable::ParserError>
    where
        Self: Sized,
    {
        Ident::parser()
            .then_ignore(LParen::parser())
            .then(existing_parser.separated_by(DelimiterOp::parser()))
            .then_ignore(RParen::parser())
            .map(|(fn_name, args)| Self { fn_name, args })
    }
}

impl Parsable for FnCall {
    fn parser() -> impl Parser<char, Self, Error = super::parsable::ParserError> {
        Self::parser_with(Expression::parser())
    }
}

#[test]
fn test_fn_call() {
    let parse = |str| FnCall::parser().parse(str);

    assert_eq!(
        parse("simple_test(123)").unwrap(),
        FnCall {
            fn_name: Ident::from_str("simple_test"),
            args: vec![Expression::NumLit(NumLit(123_f64))]
        }
    )
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub Ident);

impl Parsable for Variable {
    fn parser() -> impl Parser<char, Self, Error = ParserError> {
        Ident::parser().map(Self)
    }
}

#[test]
fn test_variable() {
    let parse = |str| Variable::parser().parse(str);

    assert_eq!(
        parse("var_name").unwrap(),
        Variable(Ident::from_str("var_name"))
    );
    assert!(parse("1test").is_err())
}

/// An expression that has a value/can return something
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Expression {
    BinExpr(Box<BinExpr>),

    FnCall(FnCall),
    Variable(Variable),

    NumLit(NumLit),
    StringLit(StringLit),
    // TODO add variables
}

impl Expression {
    pub fn parser_with_precedence(
        precedence: Precedence,
    ) -> impl Parser<char, Self, Error = ParserError> {
        recursive(|expr| {
            choice((
                // The longest possible expression needs to be parsed first.
                // Since BinExpr can start with an expression it needs to be first.
                BinExpr::parser_with_precedence(precedence)
                    .map(|bin_expr| Self::BinExpr(Box::new(bin_expr))),
                // a `FnCall` needs to be parsed first because they both start
                // with an Ident but a `FnCall` is longer.
                FnCall::parser_with(expr.clone()).map(Self::FnCall),
                Variable::parser().map(Self::Variable),
                // self contained expressions do not need a specific order
                NumLit::parser().map(Self::NumLit),
                StringLit::parser().map(Self::StringLit),
            ))
        })
    }
}

impl Parsable for Expression {
    fn parser() -> impl Parser<char, Self, Error = ParserError>
    where
        Self: Sized,
    {
        Self::parser_with_precedence(Precedence::MIN)
    }
}
