mod fn_call;
mod literal;

pub use erebus_parser::literals::Literal;
pub use fn_call::FnCall;

use std::iter::empty;

use erebus_parser::{ident::Ident, Expression as AstExpression};

use crate::{statement::NamedStatement, IntoMir};

#[derive(Debug)]
pub enum Expression<'a> {
    FnCall(FnCall<'a>),
    Literal(Literal),
    Variable(&'a NamedStatement<'a>),
}

impl<'a> IntoMir<'a> for AstExpression {
    type Target = Expression<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        empty()
    }

    fn into_mir(
        self,
        ident_resolver: impl crate::IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> Self::Target {
        #[expect(unused_variables)]
        match self {
            AstExpression::BinExpr(bin_expr) => todo!(),
            AstExpression::FnCall(fn_call) => Expression::FnCall(fn_call.into_mir(ident_resolver)),
            AstExpression::Variable(variable) => Expression::Variable(ident_resolver(variable.0)),
            AstExpression::Literal(literal) => Expression::Literal(literal),

            _ => todo!(),
        }
    }
}
