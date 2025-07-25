mod fn_def;
mod r#let;

pub use fn_def::FnDef;
pub use r#let::Let;

use std::iter::once;

use erebus_parser::{
    ident::Ident,
    statement::{
        RawTopLevelStatement, Statement as AstStatement, TopLevelStatement as AstTopLevelStatement,
    },
};

use crate::{ErrorNodeOr, IdentResolverFn, IntoMir};

/// All the Mir Nodes that could result from a path resolution (e.g. std::Tree)
#[derive(Debug)]
#[non_exhaustive]
pub enum NamedStatement<'a> {
    Let(Let<'a>),
    FnDef(FnDef<'a>),
}

impl<'a> IntoMir<'a> for AstTopLevelStatement {
    type Target = NamedStatement<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(match self.inner {
            RawTopLevelStatement::Let(r#let) => {
                NamedStatement::Let(r#let.into_mir(ident_resolver)?)
            }
            RawTopLevelStatement::FnDef(fn_def) => {
                NamedStatement::FnDef(fn_def.into_mir(ident_resolver)?)
            }

            _ => todo!(), // TODO
        })
    }

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(match &self.inner {
            RawTopLevelStatement::Let(r#let) => &r#let.left.ident,
            RawTopLevelStatement::FnDef(fn_def) => &fn_def.name,

            _ => todo!("get_ident() for {:?} not yet implemented", self.inner),
        })
    }
}

// TODO de-duplicate
impl<'a> IntoMir<'a> for AstStatement {
    type Target = NamedStatement<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(match self {
            AstStatement::Let(r#let) => &r#let.left.ident,
        })
    }

    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(match self {
            Self::Let(r#let) => NamedStatement::Let(r#let.into_mir(ident_resolver)?),
        })
    }
}
