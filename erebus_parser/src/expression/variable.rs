use chumsky::Parser;

use crate::{parsable::ParsableParser, path::Path, Parsable};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(pub Path);

impl Parsable for Variable {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Path::parser().map(Self)
    }
}

#[test]
fn test_variable() {
    assert_eq!(
        Variable::parse("var_name").unwrap(),
        Variable(Path::test_value(["var_name"]))
    );
    assert_eq!(
        Variable::parse("module_1::SOME_CONST").unwrap(),
        Variable(Path::test_value(["module_1", "SOME_CONST"]))
    );
    assert!(Variable::is_err("1test"))
}
