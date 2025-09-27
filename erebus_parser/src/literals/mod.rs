mod number;
mod string;

use chumsky::{prelude::choice, Parser};
pub use number::NumLit;
pub use string::StringLit;

use crate::{parsable::ParsableParser, Parsable};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(NumLit),
    String(StringLit),
}

impl Parsable for Literal {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        choice((
            NumLit::parser().map(Self::Number),
            StringLit::parser().map(Self::String),
        ))
    }
}

#[test]
fn test_literal() {
    assert_eq!(
        Literal::parse("123").unwrap(),
        Literal::Number(NumLit(123f64))
    );
}
