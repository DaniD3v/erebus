use bincode::{
    config::{self},
    encode_to_vec,
};
use uuid::Uuid;

use crate::r#type::{
    attrs::{Attr, FnAttr},
    Type,
};

const UUID_TYPE_NAMESPACE: Uuid = Uuid::from_bytes(*b"  Erebus Types  ");
const ID_MATCH: fn(Type, Type) -> bool = |own, other| own.id == other.id;

impl<'a> Type<'a> {
    pub const NUM_TYPE: Type<'static> =
        Type::new_builtin(Uuid::from_u128(1), &ID_MATCH, Vec::new());
    pub const STRING_TYPE: Type<'static> =
        Type::new_builtin(Uuid::from_u128(2), &ID_MATCH, Vec::new());

    pub fn fn_type(params: &'a [Type<'a>], return_type: Type<'a>) -> Self {
        let fn_attr = FnAttr {
            params,
            return_type,
        };

        Self::new_builtin(
            // generate a UUID based on fn_attr.
            // This way every function with the same signature has the same type.
            Uuid::new_v5(
                &UUID_TYPE_NAMESPACE,
                &encode_to_vec(&fn_attr, config::standard())
                    .expect("bincode failed serializing the FnAttr"),
            ),
            &ID_MATCH,
            vec![Attr::Fn(fn_attr)],
        )
    }
}
