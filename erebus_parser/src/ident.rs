use std::fmt::Display;

use chumsky::{
    prelude::{choice, just},
    text::ident,
    Parser,
};
use derivative::Derivative;

use crate::{r#type::Type, span::Span, Expression};

use super::parsable::{Parsable, ParsableParser};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialOrd, Ord, PartialEq, Eq)]
pub struct Ident {
    value: String,
    #[derivative(PartialOrd = "ignore", PartialEq = "ignore")]
    pub span: Span,
}

impl Ident {
    #[cfg(test)]
    pub fn test_value(str: &str) -> Self {
        // TODO check validity
        Self {
            value: str.to_owned(),
            span: Span::TEST_VALUE,
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Parsable for Ident {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        ident().map(|value: &str| Self {
            value: value.to_owned(),
            span: Span::TEST_VALUE,
        })
    }
}

#[test]
fn test_ident() {
    assert_eq!(
        Ident::parse("_albert132").unwrap(),
        Ident::test_value("_albert132")
    );
    // assert_eq!(Ident::parse("hyphen-var").unwrap(), Ident("hyphen".into()));
    assert!(Ident::is_err("1starts_number"));
    assert!(Ident::is_err(" starts_space123"));
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentWithType {
    pub ident: Ident,
    pub r#type: Type,
}

impl IdentWithType {
    fn parser_with<'src>(
        expression_parser: impl ParsableParser<'src, Expression>,
    ) -> impl ParsableParser<'src, Self> {
        Ident::parser()
            .then_ignore(just(":"))
            .padded()
            .then(Type::parser_with(expression_parser))
            .map(|(ident, r#type)| Self { ident, r#type })
    }
}

impl Parsable for IdentWithType {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

#[test]
fn test_ident_with_type() {
    assert_eq!(
        IdentWithType::parse("test: String").unwrap(),
        IdentWithType {
            ident: Ident::test_value("test"),
            r#type: Type::test_ident("String"),
        }
    );
    assert_eq!(
        IdentWithType::parse("test2: \n_String").unwrap(),
        IdentWithType {
            ident: Ident::test_value("test2"),
            r#type: Type::test_ident("_String"),
        }
    );
    assert!(IdentWithType::is_err("test3 : String"))
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentWithOptionalType {
    pub ident: Ident,
    pub r#type: Option<Type>,
}

impl IdentWithOptionalType {
    pub fn parser_with<'src>(
        expression_parser: impl ParsableParser<'src, Expression> + 'src,
    ) -> impl ParsableParser<'src, Self> {
        choice((
            IdentWithType::parser_with(expression_parser).map(Self::from),
            Ident::parser().map(Self::from),
        ))
    }
}

impl Parsable for IdentWithOptionalType {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

impl From<Ident> for IdentWithOptionalType {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            r#type: None,
        }
    }
}

impl From<IdentWithType> for IdentWithOptionalType {
    fn from(value: IdentWithType) -> Self {
        Self {
            ident: value.ident,
            r#type: Some(value.r#type),
        }
    }
}

#[test]
fn test_ident_with_optional_type() {
    assert_eq!(
        IdentWithOptionalType::parse("test").unwrap(),
        IdentWithOptionalType {
            ident: Ident::test_value("test"),
            r#type: None
        }
    );
    assert_eq!(
        IdentWithOptionalType::parse("str: \n\tString").unwrap(),
        IdentWithOptionalType {
            ident: Ident::test_value("str"),
            r#type: Some(Type::test_ident("String"))
        }
    )
}
