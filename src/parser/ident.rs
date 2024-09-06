use chumsky::{
    error::Simple,
    prelude::{choice, just},
    text::{ident, TextParser},
    Parser,
};

use super::parsable::Parsable;

// Alias for readability.
pub type Type = Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Parsable for Ident {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>>
    where
        Self: Sized,
    {
        ident().map(Self)
    }
}

#[test]
fn test_ident() {
    let parse = |str| Ident::parser().parse(str);

    assert_eq!(parse("_albert132 ").unwrap(), Ident("_albert132".into()));
    assert_eq!(parse("hyphen-var").unwrap(), Ident("hyphen".into()));
    assert!(parse("1starts_number").is_err());
    assert!(parse(" starts_space123").is_err());
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentWithType {
    pub ident: Ident,
    pub r#type: Type,
}

impl Parsable for IdentWithType {
    fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::prelude::Simple<char>>
    where
        Self: Sized,
    {
        Ident::parser()
            .then_ignore(just(":"))
            .padded()
            .then(Type::parser())
            .map(|(ident, r#type)| Self { ident, r#type })
    }
}

#[test]
fn test_identifier_with_type() {
    let parse = |str| IdentWithType::parser().parse(str);

    assert_eq!(
        parse("test: String").unwrap(),
        IdentWithType {
            ident: Ident("test".to_owned()),
            r#type: Ident("String".to_owned()),
        }
    );
    assert_eq!(
        parse("test2: \n_String").unwrap(),
        IdentWithType {
            ident: Ident("test2".to_owned()),
            r#type: Ident("_String".to_owned()),
        }
    );
    assert!(parse("test3 : String").is_err())
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentWithOptionalType {
    pub ident: Ident,
    pub r#type: Option<Ident>,
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

impl Parsable for IdentWithOptionalType {
    fn parser() -> impl Parser<char, Self, Error = chumsky::prelude::Simple<char>>
    where
        Self: Sized,
    {
        choice((
            IdentWithType::parser().map(Self::from),
            Ident::parser().map(Self::from),
        ))
    }
}

#[test]
fn test_ident_with_optional_type() {
    let parse = |str| IdentWithOptionalType::parser().parse(str);

    assert_eq!(
        parse("test").unwrap(),
        IdentWithOptionalType {
            ident: Ident::from_str("test"),
            r#type: None
        }
    );
    assert_eq!(
        parse("str: \n\tString").unwrap(),
        IdentWithOptionalType {
            ident: Ident::from_str("str"),
            r#type: Some(Ident::from_str("String"))
        }
    )
}
