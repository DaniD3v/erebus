use std::marker::PhantomData;

use chumsky::{
    prelude::{choice, empty},
    Parser,
};

use super::{
    expr::Expression,
    parsable::{Parsable, ParserError},
    syntax_elements::{AddExpr, DivExpr, EqualsExpr, MulExpr, SubExpr},
};

pub type Precedence = u8;

pub trait HasPrecedence {
    const PRECEDENCE: Precedence;
}

#[derive(Debug, PartialEq)]
pub struct GenericBinOp<OP: Parsable + HasPrecedence> {
    op: PhantomData<OP>,

    /// List of Expressions.
    ///
    /// The name implies that there can only be 2 Expressions but
    /// for ease of parsing they are actually stored in a chain.
    ///
    /// e.g. 1 + 2 + 3 == vec![NumLiteral(1), NumLiteral(2), NumLiteral(3)]
    expressions: Vec<Expression>,
}

impl<OP: Parsable + HasPrecedence> GenericBinOp<OP> {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self {
            op: PhantomData {},
            expressions,
        }
    }

    fn maybe_parser_with_precedence(
        current_precedence: Precedence,
    ) -> Option<impl Parser<char, Self, Error = ParserError>> {
        if OP::PRECEDENCE > current_precedence {
            Some(
                Expression::parser_with_precedence(OP::PRECEDENCE)
                    .separated_by(OP::parser())
                    .at_least(2)
                    .map(Self::new),
            )
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinExpr {
    Equals(EqualsExpr),

    Add(AddExpr),
    Sub(SubExpr),
    Mul(MulExpr),
    Div(DivExpr),
}

macro_rules! opt_parser_into_bin_expr_parser {
    ($expr_name:ident, $enum_variant:ident, $precedence:ident) => {
        $expr_name::maybe_parser_with_precedence($precedence)
            .map(|parser| parser.map(Self::$enum_variant).boxed())
    };
}

impl BinExpr {
    pub fn parser_with_precedence(
        current_precedence: Precedence,
    ) -> impl Parser<char, Self, Error = ParserError> {
        let parsers: Vec<_> = [
            // The operators with the lowest precedence need to be parsed first
            // Precedence: 1
            opt_parser_into_bin_expr_parser!(EqualsExpr, Equals, current_precedence),
            // Precedence: 2
            opt_parser_into_bin_expr_parser!(AddExpr, Add, current_precedence),
            opt_parser_into_bin_expr_parser!(SubExpr, Sub, current_precedence),
            // Precedence: 3
            opt_parser_into_bin_expr_parser!(MulExpr, Mul, current_precedence),
            opt_parser_into_bin_expr_parser!(DivExpr, Div, current_precedence),
        ]
        .into_iter()
        .flatten()
        .collect();

        if !parsers.is_empty() {
            choice(parsers).boxed()
        } else {
            // TODO chumsky bug: `https://github.com/zesterer/chumsky/issues/668`
            // this is an ugly workaround
            empty::<ParserError>().not().map(|_| unreachable!()).boxed()
        }
    }
}

impl Parsable for BinExpr {
    fn parser() -> impl Parser<char, Self, Error = ParserError> {
        Self::parser_with_precedence(Precedence::MIN)
    }
}

#[test]
fn test_bin_expr() {
    use crate::parser::literals::NumLit;
    let parse = |str| BinExpr::parser().parse(str);

    assert_eq!(
        parse("1 + 1").unwrap(),
        BinExpr::Add(AddExpr::new(vec![
            Expression::NumLit(NumLit(1_f64)),
            Expression::NumLit(NumLit(1_f64))
        ]))
    );
    assert_eq!(
        parse("1 + 1 * 1").unwrap(),
        BinExpr::Add(AddExpr::new(vec![
            Expression::NumLit(NumLit(1_f64)),
            Expression::BinExpr(Box::new(BinExpr::Mul(MulExpr::new(vec![
                Expression::NumLit(NumLit(1_f64)),
                Expression::NumLit(NumLit(1_f64))
            ]))))
        ]))
    );
    assert_eq!(
        parse("1 * 1 + 1 * 1 + 1 == 2").unwrap(),
        BinExpr::Equals(EqualsExpr::new(vec![
            Expression::BinExpr(Box::new(BinExpr::Add(AddExpr::new(vec![
                Expression::BinExpr(Box::new(BinExpr::Mul(MulExpr::new(vec![
                    Expression::NumLit(NumLit(1_f64)),
                    Expression::NumLit(NumLit(1_f64)),
                ])))),
                Expression::BinExpr(Box::new(BinExpr::Mul(MulExpr::new(vec![
                    Expression::NumLit(NumLit(1_f64)),
                    Expression::NumLit(NumLit(1_f64)),
                ])))),
                Expression::NumLit(NumLit(1_f64))
            ])))),
            Expression::NumLit(NumLit(2_f64))
        ]))
    );
}
