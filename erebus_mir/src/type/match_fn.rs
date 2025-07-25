use educe::Educe;

use crate::{r#type::Type, statement::FnDef};

/// A function that decides whether a type matches this type
#[derive(Educe, Clone)]
#[educe(Debug)]
pub(crate) enum MatchFn<'a> {
    /// A builtin match function defined in rust.
    Builtin(#[educe(Debug(ignore))] &'static (dyn Fn(Type, Type) -> bool + Sync)),
    /// A function defined within the language.
    Custom(&'a FnDef<'a>),
}
