mod crate_node;
mod statement;

pub use crate_node::Crate;

pub trait MirNode {
    type Target;
    fn named_attr(&self, name: Ident, scope: Scope) -> &Self::Target;

    fn base_ident_resolver(&self, name: Ident) -> &Self::Target {
        // Only used by children => Scope is Private
        self.named_attr(name, Scope::Private)
    }
}

trait IntoMir<'a, Input: 'a> {
    type Target;

    fn into_mir(self, ident_resolver: impl IdentResolverFn<'a, Input> + Clone) -> Self::Target;
    fn get_ident(&self) -> &Ident;
}

pub enum Scope {
    /// Accessor from the same crate, but not the same module.
    Public,

    /// Accessor is from the same module.
    Private,
}

use erebus_parser::ident::Ident;

trait IdentResolverFn<'a, R: 'a>: 'a + Fn(Ident) -> &'a R {}
impl<'a, R: 'a, T: 'a + Fn(Ident) -> &'a R> IdentResolverFn<'a, R> for T {}
