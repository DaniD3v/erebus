use std::iter::empty;

use erebus_parser::{expression::FnCall as AstFnCall, ident::Ident};

use crate::{
    expression::Expression,
    statement::{FnDef, NamedStatement},
    ErrorNodeOr, IdentResolverFn, IntoMir,
};

#[derive(Debug)]
pub struct FnCall<'a> {
    fn_obj: ErrorNodeOr<'a, &'a FnDef<'a>>,
    params: Vec<ErrorNodeOr<'a, Expression<'a>>>,
}

impl<'a> IntoMir<'a> for AstFnCall {
    type Target = FnCall<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        empty()
    }

    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(FnCall {
            fn_obj: ident_resolver(self.fn_name).map(|statement| match statement {
                NamedStatement::FnDef(_) => todo!(),
                NamedStatement::Let(_) => todo!(),
            }),
            params: self
                .params
                .into_iter()
                .map(|param| param.into_mir(ident_resolver.clone()))
                .collect(),
        })
    }
}
