use chumsky::{
    prelude::{choice, recursive},
    IterParser, Parser,
};

use crate::parser::{
    ident::Ident,
    literals::{NumLit, StringLit},
};

use super::{
    bin_ops::{BinExpr, Precedence},
    parsable::{Parsable, ParsableParser},
    statement::Statement,
    syntax_elements::{Comma, LCurly, LParen, RCurly, RParen},
};

/// Block of Code. Used in if's, matches, fn bodies, ...
#[derive(Debug, PartialEq)]
pub struct CodeScope {
    pub statements: Vec<Statement>,
    pub expr: Expression,
}

impl Parsable for CodeScope {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
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

    assert_eq!(
        CodeScope::parse("{ 1 }").unwrap(),
        CodeScope {
            statements: Vec::new(),
            expr: Expression::NumLit(NumLit(1_f64))
        }
    );

    assert_eq!(
        CodeScope::parse(
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
    fn parser_with<'src>(
        existing_parser: impl ParsableParser<'src, Expression>,
    ) -> impl ParsableParser<'src, Self> {
        Ident::parser()
            .then_ignore(LParen::parser())
            .then(existing_parser.separated_by(Comma::parser()).collect())
            .then_ignore(RParen::parser())
            .map(|(fn_name, args)| Self { fn_name, args })
    }
}

impl Parsable for FnCall {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

#[test]
fn test_fn_call() {
    assert_eq!(
        FnCall::parse("simple_test(123)").unwrap(),
        FnCall {
            fn_name: Ident::from_str("simple_test"),
            args: vec![Expression::NumLit(NumLit(123_f64))]
        }
    )
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub Ident);

impl Parsable for Variable {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Ident::parser().map(Self)
    }
}

#[test]
fn test_variable() {
    assert_eq!(
        Variable::parse("var_name").unwrap(),
        Variable(Ident::from_str("var_name"))
    );
    assert!(Variable::is_err("1test"))
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
    pub fn parser_with_precedence<'src>(precedence: Precedence) -> impl ParsableParser<'src, Self> {
        recursive(|expr| {
            choice((
                // The longest possible expression needs to be parsed first.
                // Since BinExpr can start with an expression it needs to be first.
                BinExpr::parser_with_precedence_and_parser(precedence, expr.clone())
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
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with_precedence(Precedence::MIN)
    }
}
