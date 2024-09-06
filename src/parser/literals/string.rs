use chumsky::{
    error::Simple,
    prelude::{choice, just},
    text::newline,
    Parser,
};

use crate::parser::parsable::Parsable;

#[derive(Debug, PartialEq, Eq)]
pub struct StringLit(pub String);

impl Parsable for StringLit {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>>
    where
        Self: Sized,
    {
        just('"')
            .ignore_then(
                just('\\')
                    .ignore_then(choice((
                        just('n').map(|_| Some('\n')),
                        just('\\').map(|_| Some('\\')),
                        just('"').map(|_| Some('"')),
                        newline().map(|_| None),
                    )))
                    .or(just('"').or(just('\n')).not().map(Some))
                    .repeated(),
            )
            .then_ignore(just('"'))
            .map(|iter| iter.into_iter().flatten().collect::<String>())
            .map(Self)
    }
}

#[test]
fn test_string_lit() {
    let parse = |str| StringLit::parser().parse(str);

    assert_eq!(
        parse(r#""Test\n\" newline escape""#).unwrap(),
        StringLit("Test\n\" newline escape".to_owned())
    );
    assert_eq!(
        parse(r#""Test backslash\\ escape"""#).unwrap(),
        StringLit("Test backslash\\ escape".to_owned())
    );
    assert_eq!(
        parse("\"proper multi-\\\nline\"").unwrap(),
        StringLit("proper multi-line".to_owned())
    );

    assert!(parse(r#""missing \" "#).is_err());
    assert!(parse("\"random\nnewline\"").is_err())
}
