mod code_scope;
mod fn_call;
mod variable;

use chumsky::{
    prelude::{choice, recursive},
    Parser,
};

use crate::{literals::Literal, statement::Statement};

use super::{
    bin_ops::{BinExpr, Precedence},
    parsable::{Parsable, ParsableParser},
};

pub use code_scope::CodeScope;
pub use fn_call::FnCall;
pub use variable::Variable;

/// An expression that has a value/can return something
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinExpr(Box<BinExpr>),

    FnCall(FnCall),
    Variable(Variable),

    CodeScope(Box<CodeScope>),
    Literal(Literal),
}

impl Expression {
    pub fn parser_with_precedence<'src>(precedence: Precedence) -> impl ParsableParser<'src, Self> {
        recursive(|expr_parser| {
            choice((
                // The longest possible expression needs to be parsed first.
                // Since BinExpr can start with an expression it needs to be first.
                BinExpr::parser_with_precedence_and_parser(precedence, expr_parser.clone())
                    .map(|bin_expr| Self::BinExpr(Box::new(bin_expr))),
                // a `FnCall` needs to be parsed first because they both start
                // with an Ident but a `FnCall` is longer.
                FnCall::parser_with(expr_parser.clone()).map(Self::FnCall),
                Variable::parser().map(Self::Variable),
                // self contained expressions do not need a specific order
                CodeScope::parser_with(
                    Statement::parser_with(expr_parser.clone()),
                    expr_parser.clone(),
                )
                .map(|code_scope| Self::CodeScope(Box::new(code_scope))),
                Literal::parser().map(Self::Literal),
            ))
        })
    }

    #[cfg(test)]
    pub(crate) fn num_lit(lit: f64) -> Expression {
        use crate::literals::{Literal, NumLit};
        Expression::Literal(Literal::Number(NumLit(lit)))
    }

    #[cfg(test)]
    pub(crate) fn string_lit(lit: &str) -> Expression {
        use crate::literals::{Literal, StringLit};
        Expression::Literal(Literal::String(StringLit(lit.to_owned())))
    }
}

impl Parsable for Expression {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with_precedence(Precedence::MIN)
    }
}

#[test]
fn test_expression() {
    assert_eq!(Expression::parse("1").unwrap(), Expression::num_lit(1f64))
}
