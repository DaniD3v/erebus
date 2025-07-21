use chumsky::{IterParser, Parser};

use crate::{
    parsable::ParsableParser,
    statement::Statement,
    syntax_elements::{LCurly, RCurly},
    Expression, Parsable,
};

/// Block of Code. Used in if's, matches, fn bodies, ...
#[derive(Debug, PartialEq, Clone)]
pub struct CodeScope {
    pub statements: Vec<Statement>,
    pub expr: Expression,
}

impl CodeScope {
    pub(super) fn parser_with<'src>(
        statement_parser: impl ParsableParser<'src, Statement>,
        expr_parser: impl ParsableParser<'src, Expression>,
    ) -> impl ParsableParser<'src, Self> {
        LCurly::parser()
            .ignore_then(statement_parser.repeated().collect())
            .then(expr_parser)
            .then_ignore(RCurly::parser())
            .map(|(statements, expr)| Self { statements, expr })
    }
}

impl Parsable for CodeScope {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        let expr_parser = Expression::parser();
        Self::parser_with(Statement::parser_with(expr_parser.clone()), expr_parser)
    }
}

#[test]
fn test_scope() {
    use crate::{ident::Ident, statement::Let};

    assert_eq!(
        CodeScope::parse("{ 1 }").unwrap(),
        CodeScope {
            statements: Vec::new(),
            expr: Expression::num_lit(1_f64)
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

                left: Ident::test_value("test").into(),
                right: Expression::string_lit("Statement")
            })],
            expr: Expression::string_lit("TestStatement")
        }
    );
}
