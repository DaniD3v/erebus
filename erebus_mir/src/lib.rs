#![expect(dead_code, unreachable_code)]

mod code_scope;
mod crate_node;
mod expression;
mod lazy_named_attr_map;
mod statement;
mod r#type;

use std::rc::Rc;

pub use crate_node::Crate;

use ariadne::Report;

#[derive(Debug)]
struct ErrorNode<'a> {
    report: Report<'a, Span>,
}

impl<'a> ErrorNode<'a> {
    pub fn new(report: Report<'a, Span>) -> Self {
        Self { report }
    }
}

// The Rc improves performance by reducing the size of an `ErrorNodeOr`.
// Owned ErrorNodes are sometimes necessary (e.g. a failed ident lookup)
type ErrorNodeOr<'a, T> = Result<T, Rc<ErrorNode<'a>>>;

trait MirNode {
    type Children;

    fn path_resolver(&self, path: Path) -> ErrorNodeOr<'_, &Self::Children>;
}

impl<T: MirNode> MirNode for Box<T> {
    type Children = T::Children;

    fn path_resolver(&self, path: Path) -> ErrorNodeOr<'_, &Self::Children> {
        (self as &T).path_resolver(path)
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
        ident_resolver: impl PathResolverFn<'a, Self::IdentResolverOutput> + Clone,
    ) -> ErrorNodeOr<'a, Self::Target>;
}

enum Scope {
    /// Accessor from the same crate, but not the same module.
    Public,

    /// Accessor is from the same module.
    Private,
}

use erebus_parser::{ident::Ident, Path, Span};

use crate::r#type::Type;

trait PathResolverFn<'a, R: 'a>: 'a + Fn(Path) -> ErrorNodeOr<'a, &'a R> {}
impl<'a, R: 'a, T: 'a + Fn(Path) -> ErrorNodeOr<'a, &'a R>> PathResolverFn<'a, R> for T {}
