mod attrs;
mod builtin_types;
mod match_fn;

use std::iter::once;

use erebus_parser::{
    ident::{Ident, IdentWithType as AstIdentWithType},
    Type as AstType,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    r#type::{
        attrs::{Attr, FnAttr},
        match_fn::MatchFn,
    },
    statement::NamedStatement,
    ErrorNodeOr, IntoMir,
};

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Type<'a> {
    id: Uuid,

    /// Special attrs that add compiler magic to a type
    #[serde(skip)]
    attrs: Vec<Attr<'a>>,
    /// Function that decides whether one type matches another
    #[serde(skip)]
    match_fn: MatchFn<'a>,
}

impl<'a> Type<'a> {
    fn new(match_fn: MatchFn<'a>, attrs: Vec<Attr<'a>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            attrs,
            match_fn,
        }
    }

    const fn new_builtin(
        uuid: Uuid,
        match_fn: &'static fn(Type, Type) -> bool,
        attrs: Vec<Attr<'a>>,
    ) -> Self {
        Self {
            id: uuid,
            attrs,
            match_fn: MatchFn::Builtin(match_fn),
        }
    }

    pub fn fn_attr(&self) -> Option<&FnAttr<'_>> {
        self.attrs.iter().find_map(|attr| match attr {
            Attr::Fn(fn_attr) => Some(fn_attr),
            _ => None,
        })
    }
}

impl<'a> IntoMir<'a> for AstType {
    type Target = Type<'a>;
    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        std::iter::empty()
    }

    #[expect(unused_variables)]
    fn into_mir(
        self,
        ident_resolver: impl crate::IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        todo!()
    }
}

impl<'a> Eq for Type<'a> {}
impl<'a> PartialEq for Type<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[derive(Debug)]
pub struct IdentWithType<'a> {
    ident: Ident,
    r#type: ErrorNodeOr<'a, Type<'a>>,
}

impl<'a> IntoMir<'a> for AstIdentWithType {
    type Target = IdentWithType<'a>;

    type IdentResolverOutput = NamedStatement<'a>;

    fn get_idents(&self) -> impl Iterator<Item = &Ident> {
        once(&self.ident)
    }

    fn into_mir(
        self,
        ident_resolver: impl crate::IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target> {
        Ok(IdentWithType {
            ident: self.ident,
            r#type: self.r#type.into_mir(ident_resolver),
        })
    }
}
