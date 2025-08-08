use std::iter::once;

use erebus_parser::{ident::Ident, statement::FnDef as AstFnDef};

use crate::{
    code_scope::CodeScope,
    r#type::{IdentWithType, Type},
    statement::NamedStatement,
    ErrorNodeOr, PathResolverFn, IntoMir,
};

#[derive(Debug)]
pub struct FnDef<'a> {
    name: Ident,
    body: ErrorNodeOr<'a, CodeScope<'a>>,

    params: Vec<ErrorNodeOr<'a, IdentWithType<'a>>>,
    return_type: ErrorNodeOr<'a, Type<'a>>,
}

impl<'a> IntoMir<'a> for AstFnDef {
    type Target = FnDef<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(&self.name)
    }

    fn into_mir(
        self,
        ident_resolver: impl PathResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(FnDef {
            name: self.name,
            body: self.body.into_mir(ident_resolver.clone()),
            params: self
                .params
                .into_iter()
                .map(|param| param.into_mir(ident_resolver.clone()))
                .collect(),
            return_type: todo!(),
        })
    }
}
