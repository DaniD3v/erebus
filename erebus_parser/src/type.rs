use chumsky::{
    prelude::{choice, recursive},
    IterParser, Parser,
};

use super::{
    ident::Ident,
    parsable::ParsableParser,
    syntax_elements::{Comma, FnKeyword, LParen, RParen, ReturnTypeOp},
    Parsable,
};

#[derive(Debug, PartialEq, Eq)]
pub struct FnSignatureType {
    params: Vec<TypeLiteral>,
    return_type: TypeLiteral,
}

impl FnSignatureType {
    pub fn parser_with<'src>(
        type_parser: impl ParsableParser<'src, TypeLiteral>,
    ) -> impl ParsableParser<'src, Self> {
        FnKeyword::parser()
            .ignored()
            .then(TupleType::parser_with(type_parser.clone()))
            .then(ReturnTypeOp::parser().ignore_then(type_parser).or_not())
            .map(|((_, params), return_type)| Self {
                params: params.0,
                return_type: return_type.unwrap_or_default(),
            })
    }
}

impl Parsable for FnSignatureType {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(TypeLiteral::parser())
    }
}

#[test]
fn test_fn_signature_type() {
    assert_eq!(
        FnSignatureType::parse("fn()").unwrap(),
        FnSignatureType {
            params: Vec::new(),
            return_type: TypeLiteral::Tuple(TupleType::UNIT),
        }
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleType(pub Vec<TypeLiteral>);

impl TupleType {
    pub const UNIT: Self = Self(Vec::new());

    pub fn parser_with<'src>(
        type_parser: impl ParsableParser<'src, TypeLiteral>,
    ) -> impl ParsableParser<'src, Self> {
        LParen::parser()
            .ignore_then(
                type_parser
                    .separated_by(Comma::parser())
                    .allow_trailing()
                    .collect(),
            )
            .then_ignore(RParen::parser())
            .map(Self)
    }
}

#[test]
fn test_tuple_type() {
    assert_eq!(TupleType::parse("()").unwrap(), TupleType::UNIT);
    assert_eq!(
        TupleType::parse("((()))").unwrap(),
        TupleType(vec![TypeLiteral::Tuple(TupleType(vec![
            TypeLiteral::Tuple(TupleType::UNIT)
        ]))])
    );

    assert!(TupleType::is_err("(,)"))
}

impl Parsable for TupleType {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(TypeLiteral::parser())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeLiteral {
    /// Either refers to a struct or to a generic parameter
    Ident(Ident),
    Fn(Box<FnSignatureType>),
    Tuple(TupleType),
    // TODO array & tuple literals // recursive type. This can indirectly have generics
    // TODO trait obj // this can also have generics
}

impl Default for TypeLiteral {
    fn default() -> Self {
        Self::Tuple(TupleType::UNIT)
    }
}

impl Parsable for TypeLiteral {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        recursive(|type_parser| {
            choice((
                FnSignatureType::parser_with(type_parser.clone()).map(|t| Self::Fn(Box::new(t))),
                TupleType::parser_with(type_parser).map(Self::Tuple),
                // Ident must be parsed last because e.g. fn could be considered a keyword
                Ident::parser().map(Self::Ident),
            ))
        })
    }
}

#[test]
fn test_type() {
    assert_eq!(
        TypeLiteral::parse("String").unwrap(),
        TypeLiteral::Ident(Ident::from_str("String"))
    );
    assert_eq!(
        TypeLiteral::parse("fn(int,) -> String").unwrap(),
        TypeLiteral::Fn(Box::new(FnSignatureType {
            params: vec![TypeLiteral::Ident(Ident::from_str("int"))],
            return_type: TypeLiteral::Ident(Ident::from_str("String"))
        }))
    );
    assert_eq!(
        TypeLiteral::parse("(String, (int, T))").unwrap(),
        TypeLiteral::Tuple(TupleType(vec![
            TypeLiteral::Ident(Ident::from_str("String")),
            TypeLiteral::Tuple(TupleType(vec![
                TypeLiteral::Ident(Ident::from_str("int")),
                TypeLiteral::Ident(Ident::from_str("T")),
            ]))
        ]))
    )
}
