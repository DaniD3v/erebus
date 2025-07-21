use crate::{
    ident::Ident,
    parsable::ParsableParser,
    syntax_elements::{Comma, LParen, RParen},
    Expression, Parsable,
};
use chumsky::{IterParser, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct FnCall {
    pub fn_name: Ident,
    pub params: Vec<Expression>,
}

impl FnCall {
    pub(super) fn parser_with<'src>(
        expr_parser: impl ParsableParser<'src, Expression>,
    ) -> impl ParsableParser<'src, Self> {
        Ident::parser()
            .then_ignore(LParen::parser())
            .then(expr_parser.separated_by(Comma::parser()).collect())
            .then_ignore(RParen::parser())
            .map(|(fn_name, args)| Self {
                fn_name,
                params: args,
            })
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
            fn_name: Ident::test_value("simple_test"),
            params: vec![Expression::num_lit(123_f64)]
        }
    )
}
