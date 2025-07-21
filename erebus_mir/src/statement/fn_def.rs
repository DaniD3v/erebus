use std::iter::once;

use erebus_parser::{ident::Ident, statement::FnDef as AstFnDef};

use crate::{
    code_scope::CodeScope,
    r#type::{IdentWithType, Type},
    statement::NamedStatement,
    IdentResolverFn, IntoMir,
};

#[derive(Debug)]
pub struct FnDef<'a> {
    name: Ident,
    body: CodeScope<'a>,

    params: Vec<IdentWithType<'a>>,
    return_type: Type<'a>,
}

impl<'a> IntoMir<'a> for AstFnDef {
    type Target = FnDef<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(&self.name)
    }

    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> Self::Target {
        FnDef {
            name: self.name,
            body: self.body.into_mir(ident_resolver.clone()),
            params: self
                .params
                .into_iter()
                .map(|param| param.into_mir(ident_resolver.clone()))
                .collect(),
            return_type: todo!(),
        }
    }
}
