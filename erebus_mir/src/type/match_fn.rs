use derive_debug::Dbg;

use crate::{r#type::Type, statement::FnDef};

/// A function that decides whether a type matches this type
#[derive(Dbg, Clone)]
pub(crate) enum MatchFn<'a> {
    /// A builtin match function defined in rust.
    #[dbg(skip)]
    Builtin(&'static (dyn Fn(Type, Type) -> bool + Sync)),
    /// A function defined within the language.
    Custom(&'a FnDef<'a>),
}
