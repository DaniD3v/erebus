use erebus_parser::{
    ident::Ident,
    statement::{
        RawTopLevelStatement, StructDef as AstStructDef, TopLevelStatement as AstTopLevelStatement,
    },
};

use crate::{IdentResolverFn, IntoMir};

pub struct StructDef<'a> {
    #[expect(dead_code)]
    parent_ident_resolver: Box<dyn IdentResolverFn<'a, NamedStatement<'a>>>,
}

impl<'a> std::fmt::Debug for StructDef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructDef").finish()
    }
}

impl<'a> IntoMir<'a, NamedStatement<'a>> for AstStructDef {
    type Target = StructDef<'a>;

    #[expect(unused_variables)]
    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, NamedStatement<'a>>,
    ) -> Self::Target {
        todo!()
    }

    fn get_ident(&self) -> &Ident {
        todo!()
    }
}

/// All the Mir Nodes that could result from a path resolution (e.g. std::Tree)
#[derive(Debug)]
#[non_exhaustive]
pub enum NamedStatement<'a> {
    StructDef(StructDef<'a>),
}

impl<'a> IntoMir<'a, NamedStatement<'a>> for AstTopLevelStatement {
    type Target = NamedStatement<'a>;

    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, NamedStatement<'a>> + Clone,
    ) -> Self::Target {
        match self.inner {
            RawTopLevelStatement::StructDef(statement) => {
                NamedStatement::StructDef(statement.into_mir(ident_resolver))
            }

            _ => todo!(), // TODO
        }
    }

    fn get_ident(&self) -> &Ident {
        match &self.inner {
            RawTopLevelStatement::StructDef(struct_def) => &struct_def.name,
            RawTopLevelStatement::Let(let_statement) => &let_statement.left.ident,
            RawTopLevelStatement::FnDef(fn_def) => &fn_def.name,

            _ => todo!("get_ident() for {:?} not yet implemented", self.inner),
        }
    }
}
