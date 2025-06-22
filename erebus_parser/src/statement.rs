use std::fmt::Debug;

use chumsky::{prelude::choice, text::whitespace, IterParser, Parser};
use enum_dispatch::enum_dispatch;

use crate::ident::{Ident, IdentWithType};

use super::{
    expr::{CodeScope, Expression},
    ident::IdentWithOptionalType,
    parsable::{Parsable, ParsableParser},
    r#type::TypeLiteral,
    syntax_elements::{
        AssignmentOp, Comma, FnKeyword, LCurly, LParen, LetKeyword, MutModifier, PubModifier,
        RCurly, RParen, ReturnTypeOp, Semicolon, StructKeyword,
    },
};

#[derive(Debug, PartialEq)]
pub struct MaybePublic<T> {
    is_pub: bool,
    pub inner: T,
}

impl<T: Parsable> Parsable for MaybePublic<T> {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        PubModifier::parser()
            .or_not()
            .then(T::parser())
            .map(|(pub_modifier, inner)| Self {
                is_pub: pub_modifier.is_some(),
                inner,
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub is_mut: bool,

    pub left: IdentWithOptionalType,
    pub right: Expression,
}

impl Parsable for Let {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        LetKeyword::parser()
            .then(MutModifier::parser().padded().or_not())
            .then(IdentWithOptionalType::parser().padded())
            .then_ignore(AssignmentOp::parser())
            .then(Expression::parser())
            .map(|(((_, mut_modifier), left), right)| Self {
                is_mut: mut_modifier.is_some(),

                left,
                right,
            })
    }
}

#[test]
fn test_let() {
    assert_eq!(
        Let::parse("let _test = 123").unwrap(),
        Let {
            is_mut: false,

            left: Ident::test_value("_test").into(),
            right: Expression::num_lit(123_f64),
        }
    );
    assert_eq!(
        Let::parse("let mut o:String=\"helloTest\"").unwrap(),
        Let {
            is_mut: true,

            left: IdentWithType {
                ident: Ident::test_value("o"),
                r#type: TypeLiteral::Ident(Ident::test_value("String")),
            }
            .into(),
            right: Expression::string_lit("helloTest"),
        }
    );

    assert!(Let::is_err("letmut a = 321"));
    assert!(Let::is_err("let mut 1 = 321"));
    assert!(Let::is_err("let mut a == 321"));
}

#[derive(Debug, PartialEq)]
pub struct FnDef {
    pub name: Ident,

    params: Vec<IdentWithType>,
    return_type: TypeLiteral,

    body: CodeScope,
}

impl Parsable for FnDef {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        FnKeyword::parser()
            .ignored()
            .then_ignore(whitespace())
            .ignore_then(Ident::parser())
            .then_ignore(LParen::parser())
            .then(
                IdentWithType::parser()
                    .separated_by(Comma::parser())
                    .collect(),
            )
            .then_ignore(RParen::parser())
            .then_ignore(ReturnTypeOp::parser().padded())
            .then(TypeLiteral::parser())
            .then(CodeScope::parser())
            .map(|(((name, params), return_type), body)| Self {
                name,

                params,
                return_type,

                body,
            })
    }
}

#[test]
fn test_fn() {
    assert_eq!(
        FnDef::parse("fn basic_test_fn(arg1: int) -> String { \"test\" }").unwrap(),
        FnDef {
            name: Ident::test_value("basic_test_fn"),

            params: vec![IdentWithType {
                ident: Ident::test_value("arg1"),
                r#type: TypeLiteral::Ident(Ident::test_value("int")),
            }],
            return_type: TypeLiteral::Ident(Ident::test_value("String")),

            body: CodeScope {
                statements: Vec::new(),
                expr: Expression::string_lit("test")
            }
        }
    )
}

#[derive(Debug, PartialEq)]
pub struct StructDef {
    pub name: Ident,
    fields: Vec<IdentWithType>,
}

impl Parsable for StructDef {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        StructKeyword::parser()
            .ignore_then(Ident::parser().padded())
            .then_ignore(LCurly::parser())
            .then(
                IdentWithType::parser()
                    .separated_by(Comma::parser())
                    .allow_trailing()
                    .collect(),
            )
            .then_ignore(RCurly::parser())
            .map(|(name, fields)| Self { name, fields })
    }
}

#[test]
fn test_struct_def() {
    assert_eq!(
        StructDef::parse("struct SimpleTest { a: int, b: String, }").unwrap(),
        StructDef {
            name: Ident::test_value("SimpleTest"),
            fields: vec![
                IdentWithType {
                    ident: Ident::test_value("a"),
                    r#type: TypeLiteral::Ident(Ident::test_value("int")),
                },
                IdentWithType {
                    ident: Ident::test_value("b"),
                    r#type: TypeLiteral::Ident(Ident::test_value("String")),
                }
            ]
        }
    )
}

// TODO test
pub type TopLevelStatement = MaybePublic<RawTopLevelStatement>;

/// The statements you can put at the outermost scope of each file.
#[enum_dispatch]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum RawTopLevelStatement {
    Let,
    FnDef,
    StructDef,
}

impl Parsable for RawTopLevelStatement {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        choice((
            Let::parser()
                .then_ignore(Semicolon::parser())
                .map(Self::Let),
            FnDef::parser().map(Self::FnDef),
            StructDef::parser().map(Self::StructDef),
        ))
        .padded()
    }
}

/// Something that cannot return a value.
///
/// Always delimited with a semicolon.
#[enum_dispatch]
#[derive(Debug, PartialEq)]
pub enum Statement {
    Let,
}

impl Parsable for Statement {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Let::parser()
            .then_ignore(Semicolon::parser())
            .map(Self::Let)
    }
}

#[test]
fn test_statement() {
    assert_eq!(
        Statement::parse("let var = \"simple_let\";").unwrap(),
        Statement::Let(Let {
            is_mut: false,
            left: Ident::test_value("var").into(),
            right: Expression::string_lit("simple_let")
        })
    );

    assert!(Statement::is_err("let missing_semicolon = 1"));
}
