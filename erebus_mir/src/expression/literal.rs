use erebus_parser::literals::Literal;

use crate::{r#type::Type, Typed};

impl Typed<'static> for Literal {
    fn get_type(&self) -> Type<'static> {
        match self {
            Literal::Number(_) => Type::NUM_TYPE,
            Literal::String(_) => Type::STRING_TYPE,
        }
    }
}
