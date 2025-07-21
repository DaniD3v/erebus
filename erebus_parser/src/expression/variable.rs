use chumsky::Parser;

use crate::{ident::Ident, parsable::ParsableParser, Parsable};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(pub Ident);

impl Parsable for Variable {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Ident::parser().map(Self)
    }
}

#[test]
fn test_variable() {
    assert_eq!(
        Variable::parse("var_name").unwrap(),
        Variable(Ident::test_value("var_name"))
    );
    assert!(Variable::is_err("1test"))
}
