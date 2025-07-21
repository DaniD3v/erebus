use std::convert::Infallible;

use bincode::Encode;

use crate::r#type::Type;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum Attr<'a> {
    Fn(FnAttr<'a>),

    /// This is a fake attribute! This is unused by actual code.
    /// non_exhaustive does not work in the local crate
    /// however I still want to enforce matching a wildcard
    UnusedUnmatchable(Infallible),
}

#[derive(Debug, Clone, Encode)]
pub(crate) struct FnAttr<'a> {
    #[bincode(with_serde)]
    pub params: &'a [Type<'a>],
    #[bincode(with_serde)]
    pub return_type: Type<'a>,
}
