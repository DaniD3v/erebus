use chumsky::{
    prelude::{choice, just, none_of},
    text::newline,
    IterParser, Parser,
};

use crate::parsable::{Parsable, ParsableParser};

#[derive(Debug, PartialEq, Eq)]
pub struct StringLit(pub String);

impl Parsable for StringLit {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        just('"')
            .ignore_then(
                just('\\')
                    .ignore_then(choice((
                        just('n').to(Some('\n')),
                        just('\\').to(Some('\\')),
                        just('"').to(Some('"')),
                        newline().to(None),
                    )))
                    .or(none_of("\"\n").map(Some))
                    .repeated()
                    .collect(),
            )
            .then_ignore(just('"'))
            .map(|iter: Vec<_>| iter.into_iter().flatten().collect::<String>())
            .map(Self)
    }
}

#[test]
fn test_string_lit() {
    assert_eq!(
        StringLit::parse(r#""Test\n\" newline escape""#).unwrap(),
        StringLit("Test\n\" newline escape".to_owned())
    );
    assert_eq!(
        StringLit::parse(r#""Test backslash\\ escape""#).unwrap(),
        StringLit("Test backslash\\ escape".to_owned())
    );
    assert_eq!(
        StringLit::parse("\"proper multi-\\\nline\"").unwrap(),
        StringLit("proper multi-line".to_owned())
    );

    assert!(StringLit::is_err(r#""missing \" "#));
    assert!(StringLit::is_err("\"random\nnewline\""))
}
