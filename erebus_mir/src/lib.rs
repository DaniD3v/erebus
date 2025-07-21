#![expect(dead_code, unreachable_code)]

mod code_scope;
mod crate_node;
mod expression;
mod lazy_named_attr_map;
mod statement;
mod r#type;

pub use crate_node::Crate;

trait MirNode {
    type Children;

    fn ident_resolver(&self, name: Ident) -> &Self::Children;
}

impl<T: MirNode> MirNode for Box<T> {
    type Children = T::Children;

    fn ident_resolver(&self, name: Ident) -> &Self::Children {
        (self as &T).ident_resolver(name)
    }
}

trait Typed<'a> {
    fn get_type(&self) -> Type<'a>;
}

trait IntoMir<'a> {
    type Target;
    /// Type that the IdentResolver should emit
    type IdentResolverOutput: 'a;

    fn get_idents(&self) -> impl Iterator<Item = &Ident>;
    fn into_mir(
        self,
        ident_resolver: impl IdentResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> Self::Target;
}

enum Scope {
    /// Accessor from the same crate, but not the same module.
    Public,

    /// Accessor is from the same module.
    Private,
}

use erebus_parser::ident::Ident;

use crate::r#type::Type;

trait IdentResolverFn<'a, R: 'a>: 'a + Fn(Ident) -> &'a R {}
impl<'a, R: 'a, T: 'a + Fn(Ident) -> &'a R> IdentResolverFn<'a, R> for T {}
