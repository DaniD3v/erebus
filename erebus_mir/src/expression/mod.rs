mod fn_call;
mod literal;

pub use erebus_parser::literals::Literal;
pub use fn_call::FnCall;

use std::iter::empty;

use erebus_parser::{ident::Ident, Expression as AstExpression};

use crate::{statement::NamedStatement, ErrorNodeOr, IntoMir, PathResolverFn};

#[derive(Debug)]
pub enum Expression<'a> {
    FnCall(ErrorNodeOr<'a, FnCall<'a>>),
    Literal(Literal),
    Variable(ErrorNodeOr<'a, &'a NamedStatement<'a>>),
}

impl<'a> IntoMir<'a> for AstExpression {
    type Target = Expression<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        empty()
    }

    fn into_mir(
        self,
        ident_resolver: impl PathResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        #[expect(unused_variables)]
        Ok(match self {
            AstExpression::BinExpr(bin_expr) => todo!(),
            AstExpression::FnCall(fn_call) => Expression::FnCall(fn_call.into_mir(ident_resolver)),
            AstExpression::Variable(variable) => Expression::Variable(ident_resolver(variable.0)),
            AstExpression::Literal(literal) => Expression::Literal(literal),

            _ => todo!(),
        })
    }
}
