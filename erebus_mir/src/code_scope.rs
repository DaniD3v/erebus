use std::iter::empty;

use erebus_parser::expression::CodeScope as AstCodeScope;

use crate::{expression::Expression, statement::NamedStatement, ErrorNodeOr, IntoMir};

#[derive(Debug)]
pub struct CodeScope<'a> {
    statements: Vec<ErrorNodeOr<'a, NamedStatement<'a>>>,
    expr: ErrorNodeOr<'a, Expression<'a>>,
}

impl<'a> IntoMir<'a> for AstCodeScope {
    type Target = CodeScope<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &erebus_parser::ident::Ident> {
        empty()
    }

    fn into_mir(
        self,
        ident_resolver: impl crate::IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, CodeScope<'a>> {
        Ok(Self::Target {
            statements: self
                .statements
                .into_iter()
                .map(|statement| statement.into_mir(ident_resolver.clone()))
                .collect(),
            expr: self.expr.into_mir(ident_resolver),
        })
    }
}
