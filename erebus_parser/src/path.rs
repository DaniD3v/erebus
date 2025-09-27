use std::fmt::{Display, Formatter};

use chumsky::{IterParser, Parser};
use educe::Educe;

use crate::{ident::Ident, parsable::ParsableParser, syntax_elements::PathSegment, Parsable, Span};

#[derive(Educe, Debug, Clone)]
#[educe(PartialOrd, Ord, PartialEq, Eq)]
pub struct Path {
    segments: Vec<Ident>,

    #[educe(PartialOrd(ignore), PartialEq(ignore))]
    pub span: Span,
}

impl Path {
    #[cfg(test)]
    pub fn test_value<const S: usize>(segments: [&str; S]) -> Self {
        let segments = segments
            .into_iter()
            .map(|ident_str| Ident::test_value(ident_str))
            .collect();

        Self {
            segments,
            span: Span::TEST_VALUE,
        }
    }
}

impl Parsable for Path {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Ident::parser()
            .separated_by(PathSegment::parser())
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|idents, ctx| Self {
                segments: idents,
                span: ctx.span(),
            })
    }
}

impl From<Ident> for Path {
    fn from(ident: Ident) -> Self {
        Self {
            segments: vec![ident.clone()],
            span: ident.span,
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .segments
                .iter()
                .fold(self.segments[0].value().to_owned(), |acc, segment| {
                    format!("{acc}::{}", segment.value())
                }),
        )
    }
}

#[test]
fn test_path() {
    assert_eq!(
        Path::parse("module1::var_name").unwrap(),
        Path::test_value(["module1", "var_name"])
    );
    assert_eq!(
        Path::parse("test_mod::fn_upper").unwrap(),
        Path::test_value(["test_mod", "fn_upper"])
    );
    assert_eq!(
        Path::parse("print").unwrap(),
        Ident::test_value("print").into(),
    );
    assert!(Path::is_err("1mod::abc"))
}
