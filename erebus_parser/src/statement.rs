use chumsky::{prelude::choice, text::whitespace, IterParser, Parser};

use crate::ident::{Ident, IdentWithType};

use super::{
    expression::{CodeScope, Expression},
    ident::IdentWithOptionalType,
    parsable::{Parsable, ParsableParser},
    syntax_elements::{
        AssignmentOp, Comma, FnKeyword, LParen, LetKeyword, MutModifier, PubModifier, RParen,
        ReturnTypeOp, Semicolon,
    },
};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Let {
    pub is_mut: bool,

    pub left: IdentWithOptionalType,
    pub right: Expression,
}

impl Let {
    fn parser_with<'src>(
        expression_parser: impl ParsableParser<'src, Expression> + 'src,
    ) -> impl ParsableParser<'src, Self> {
        LetKeyword::parser()
            .then(MutModifier::parser().padded().or_not())
            .then(IdentWithOptionalType::parser_with(expression_parser.clone()).padded())
            .then_ignore(AssignmentOp::parser())
            .then(expression_parser)
            .map(|(((_, mut_modifier), left), right)| Self {
                is_mut: mut_modifier.is_some(),

                left,
                right,
            })
    }
}

impl Parsable for Let {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

#[test]
fn test_let() {
    use crate::Type;

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
                r#type: Type::test_ident("String"),
            }
            .into(),
            right: Expression::string_lit("helloTest"),
        }
    );

    assert!(Let::is_err("letmut a = 321"));
    assert!(Let::is_err("let mut 1 = 321"));
    assert!(Let::is_err("let mut a == 321"));
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnDef {
    pub name: Ident,
    pub body: CodeScope,

    pub params: Vec<IdentWithType>,
    pub return_type: Ident,
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
            .then(Ident::parser())
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
    use crate::Type;

    assert_eq!(
        FnDef::parse("fn basic_test_fn(arg1: int) -> String { \"test\" }").unwrap(),
        FnDef {
            name: Ident::test_value("basic_test_fn"),

            params: vec![IdentWithType {
                ident: Ident::test_value("arg1"),
                r#type: Type::test_ident("int"),
            }],
            return_type: Ident::test_value("String"),

            body: CodeScope {
                statements: Vec::new(),
                expr: Expression::string_lit("test")
            }
        }
    )
}

// TODO test
pub type TopLevelStatement = MaybePublic<RawTopLevelStatement>;

/// The statements you can put at the outermost scope of each file.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum RawTopLevelStatement {
    Let(Let),
    FnDef(FnDef),
}

impl Parsable for RawTopLevelStatement {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        choice((
            Let::parser()
                .then_ignore(Semicolon::parser())
                .map(Self::Let),
            FnDef::parser().map(Self::FnDef),
        ))
        .padded()
    }
}

/// Something that cannot return a value.
///
/// Always delimited with a semicolon.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(Let),
}

impl Statement {
    pub fn parser_with<'src>(
        expression_parser: impl ParsableParser<'src, Expression> + 'src,
    ) -> impl ParsableParser<'src, Self> {
        Let::parser_with(expression_parser)
            .then_ignore(Semicolon::parser())
            .map(Self::Let)
    }
}

impl Parsable for Statement {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
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
