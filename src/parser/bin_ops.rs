use std::fmt::Debug;
use std::marker::PhantomData;

use chumsky::{prelude::choice, Parser};

use super::{
    expr::Expression,
    parsable::{Parsable, ParsableParser},
    syntax_elements::{AddExpr, DivExpr, EqualsExpr, MulExpr, SubExpr},
};

pub type Precedence = u8;

pub trait HasPrecedence {
    const PRECEDENCE: Precedence;
}

#[derive(Debug, PartialEq)]
pub struct GenericBinOp<OP: Parsable + HasPrecedence> {
    op: PhantomData<OP>,
    expressions: [Expression; 2],
}

impl<OP: Parsable + HasPrecedence + Debug> GenericBinOp<OP> {
    pub fn new(expressions: [Expression; 2]) -> Self {
        Self {
            op: PhantomData {},
            expressions,
        }
    }

    fn maybe_parser_with_precedence_and_parser<'src>(
        current_precedence: Precedence,
        // this parser must be of `current_precedence`
        expression_parser: impl ParsableParser<'src, Expression>,
    ) -> Option<impl ParsableParser<'src, Self>> {
        if OP::PRECEDENCE >= current_precedence {
            Some(
                // We're not allowed to search for an expression with the same operator to the left.
                // This prevents infinite recursion.
                Expression::parser_with_precedence(OP::PRECEDENCE + 1)
                    .then_ignore(OP::parser())
                    // It's not possible to cause infinite recursion on the right side.
                    .then(expression_parser)
                    .map(|(expression0, expression1)| Self::new([expression0, expression1])),
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

impl BinExpr {
    pub fn parser_with_precedence_and_parser<'src>(
        current_precedence: Precedence,
        // this parser must be of `current_precedence`
        expression_parser: impl ParsableParser<'src, Expression> + 'src,
    ) -> impl ParsableParser<'src, Self> {
        macro_rules! parse_as_bin_expr {
            ($expr_name:ident) => {
                $expr_name::maybe_parser_with_precedence_and_parser(
                    current_precedence,
                    expression_parser.clone(),
                )
                .map(|parser| {
                    parser
                        .map(|expressions| $expr_name::into_bin_expr(expressions))
                        .boxed()
                })
            };
        }

        let parsers: Vec<_> = [
            // The operators with the lowest precedence need to be parsed first
            // Precedence: 1
            parse_as_bin_expr!(EqualsExpr),
            // Precedence: 2
            parse_as_bin_expr!(AddExpr),
            parse_as_bin_expr!(SubExpr),
            // Precedence: 3
            parse_as_bin_expr!(MulExpr),
            parse_as_bin_expr!(DivExpr),
        ]
        .into_iter()
        .flatten()
        .collect();

        choice(parsers).boxed()
    }
}

impl Parsable for BinExpr {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with_precedence_and_parser(
            Precedence::MIN,
            Expression::parser_with_precedence(Precedence::MIN),
        )
    }
}

#[test]
fn test_bin_expr() {
    use crate::parser::literals::NumLit;

    assert_eq!(
        BinExpr::parse("1 + 1").unwrap(),
        BinExpr::Add(AddExpr::new([
            Expression::NumLit(NumLit(1_f64)),
            Expression::NumLit(NumLit(1_f64))
        ]))
    );
    assert_eq!(
        BinExpr::parse("1 + 2 * 3 / 4").unwrap(),
        BinExpr::Add(AddExpr::new([
            Expression::NumLit(NumLit(1_f64)),
            MulExpr::as_expr([
                Expression::NumLit(NumLit(2_f64)),
                DivExpr::as_expr([
                    Expression::NumLit(NumLit(3_f64)),
                    Expression::NumLit(NumLit(4_f64))
                ])
            ])
        ]))
    );
    assert_eq!(
        BinExpr::parse("3 + 2 - 1").unwrap(),
        BinExpr::Add(AddExpr::new([
            Expression::NumLit(NumLit(3_f64)),
            SubExpr::as_expr([
                Expression::NumLit(NumLit(2_f64)),
                Expression::NumLit(NumLit(1_f64)),
            ])
        ]))
    );
    assert_eq!(
        // use expression parser to make sure it also works there
        Expression::parse("1 * 2 + 3 * 4 + 5 == 6").unwrap(),
        EqualsExpr::as_expr([
            AddExpr::as_expr([
                MulExpr::as_expr([
                    Expression::NumLit(NumLit(1_f64)),
                    Expression::NumLit(NumLit(2_f64)),
                ]),
                AddExpr::as_expr([
                    MulExpr::as_expr([
                        Expression::NumLit(NumLit(3_f64)),
                        Expression::NumLit(NumLit(4_f64)),
                    ]),
                    Expression::NumLit(NumLit(5_f64))
                ])
            ]),
            Expression::NumLit(NumLit(6_f64))
        ])
    );
}
