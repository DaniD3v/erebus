use std::iter::empty;

use erebus_parser::{expression::FnCall as AstFnCall, ident::Ident};

use crate::{
    expression::Expression,
    statement::{FnDef, NamedStatement},
    IntoMir,
};

#[derive(Debug)]
pub struct FnCall<'a> {
    fn_obj: &'a FnDef<'a>,
    params: Vec<Expression<'a>>,
}

impl<'a> IntoMir<'a> for AstFnCall {
    type Target = FnCall<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        empty()
    }

    fn into_mir(
        self,
        ident_resolver: impl crate::IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> Self::Target {
        FnCall {
            fn_obj: match ident_resolver(self.fn_name) {
                NamedStatement::FnDef(_) => todo!(),
                NamedStatement::Let(_) => todo!(),
            },
            params: self
                .params
                .into_iter()
                .map(|param| param.into_mir(ident_resolver.clone()))
                .collect(),
        }
    }
}
