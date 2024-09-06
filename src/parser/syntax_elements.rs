use chumsky::{
    prelude::just,
    text::{keyword, TextParser},
    Parser,
};

use super::{
    bin_ops::{GenericBinOp, HasPrecedence, Precedence},
    parsable::Parsable,
    ParserError,
};

macro_rules! generate_parsable {
    ($ident:ident, $impl:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $ident;

        impl Parsable for $ident {
            fn parser() -> impl chumsky::Parser<char, Self, Error = ParserError>
            where
                Self: Sized,
            {
                $impl
            }
        }
    };
}

macro_rules! generate_keyword_parsable {
    ($ident:ident, $str_repr:literal) => {
        generate_parsable! {$ident, keyword($str_repr).map(|()| Self {})}
    };
}

macro_rules! generate_operator_parsable {
    ($ident:ident, $str_repr:literal) => {
        generate_parsable! {$ident, just($str_repr).padded().map(|_| Self {})}
    };
}

macro_rules! generate_binary_operator_parsable {
    ($op_name:ident, $expr_name:ident, $precedence:literal, $str_repr:literal) => {
        pub type $expr_name = GenericBinOp<$op_name>;
        generate_operator_parsable! {$op_name, $str_repr}

        impl HasPrecedence for $op_name {
            const PRECEDENCE: Precedence = $precedence;
        }
    };
}

generate_operator_parsable! {AssignmentOp, '='}
generate_operator_parsable! {ReturnTypeOp, "->"}
generate_operator_parsable! {DotOp, '.'}
generate_operator_parsable! {DelimiterOp, ','}
generate_operator_parsable! {Semicolon, ';'}
generate_operator_parsable! {MacroCallOp, '!'}

// Precedence needs to start at 1, because 0 can parse all Expressions.
generate_binary_operator_parsable! {EqualsOp, EqualsExpr, 1, "=="}
generate_binary_operator_parsable! {AddOp, AddExpr, 2, '+'}
generate_binary_operator_parsable! {SubOp, SubExpr, 2, '-'}
generate_binary_operator_parsable! {MulOp, MulExpr, 3, '*'}
generate_binary_operator_parsable! {DivOp, DivExpr, 3, '/'}

generate_operator_parsable! {LCurly, '{'}
generate_operator_parsable! {RCurly, '}'}
generate_operator_parsable! {LParen, '('}
generate_operator_parsable! {RParen, ')'}

generate_keyword_parsable! {MutModifier, "mut"}
generate_keyword_parsable! {PubModifier, "pub"}

generate_keyword_parsable! {LetKeyword, "let"}
generate_keyword_parsable! {FnKeyword, "fn"}
