use std::iter::once;

use erebus_parser::{ident::Ident, statement::Let as AstLet};

use crate::{
    expression::Expression, r#type::Type, statement::NamedStatement, ErrorNodeOr, IntoMir,
    PathResolverFn,
};

#[derive(Debug)]
pub struct Let<'a> {
    ident: Ident,
    r#type: Option<ErrorNodeOr<'a, Type<'a>>>,
    value: ErrorNodeOr<'a, Expression<'a>>,
}

impl<'a> Let<'a> {
    /// Constructs a fake let that is used to define a builtin
    pub fn fake_let() {
        todo!()
    }
}

impl<'a> IntoMir<'a> for AstLet {
    type Target = Let<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(&self.left.ident)
    }

    fn into_mir(
        self,
        ident_resolver: impl PathResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(Let {
            ident: self.left.ident,
            r#type: self.left.r#type.map(|t| t.into_mir(ident_resolver.clone())),

            value: self.right.into_mir(ident_resolver),
        })
    }
}
