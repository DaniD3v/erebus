use chumsky::{
    prelude::{choice, just},
    text::ident,
    IterParser, Parser,
};

use super::parsable::{Parsable, ParsableParser};

// Alias for readability.
pub type Type = Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
    #[cfg(test)]
    pub fn from_str(str: &str) -> Self {
        Ident(str.to_owned())
    }
}

impl Parsable for Ident {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        // TODO this is a hack.
        // Allowing underscores in ident is already fixed in 8c8b82beb2a1b55ccee133266bbd52ffc0eb27a2
        just('_')
            .repeated()
            .collect::<String>()
            .then(ident())
            .map(|(mut underscores, str)| {
                underscores.push_str(str);
                Self(underscores)
            })
    }
}

#[test]
fn test_ident() {
    assert_eq!(
        Ident::parse("_albert132").unwrap(),
        Ident("_albert132".into())
    );
    // assert_eq!(Ident::parse("hyphen-var").unwrap(), Ident("hyphen".into()));
    assert!(Ident::is_err("1starts_number"));
    assert!(Ident::is_err(" starts_space123"));
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentWithType {
    pub ident: Ident,
    pub r#type: Type,
}

impl Parsable for IdentWithType {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Ident::parser()
            .then_ignore(just(":"))
            .padded()
            .then(Type::parser())
            .map(|(ident, r#type)| Self { ident, r#type })
    }
}

#[test]
fn test_identifier_with_type() {
    assert_eq!(
        IdentWithType::parse("test: String").unwrap(),
        IdentWithType {
            ident: Ident::from_str("test"),
            r#type: Ident::from_str("String"),
        }
    );
    assert_eq!(
        IdentWithType::parse("test2: \n_String").unwrap(),
        IdentWithType {
            ident: Ident::from_str("test2"),
            r#type: Ident::from_str("_String"),
        }
    );
    assert!(IdentWithType::is_err("test3 : String"))
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
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        choice((
            IdentWithType::parser().map(Self::from),
            Ident::parser().map(Self::from),
        ))
    }
}

#[test]
fn test_ident_with_optional_type() {
    assert_eq!(
        IdentWithOptionalType::parse("test").unwrap(),
        IdentWithOptionalType {
            ident: Ident::from_str("test"),
            r#type: None
        }
    );
    assert_eq!(
        IdentWithOptionalType::parse("str: \n\tString").unwrap(),
        IdentWithOptionalType {
            ident: Ident::from_str("str"),
            r#type: Some(Ident::from_str("String"))
        }
    )
}
