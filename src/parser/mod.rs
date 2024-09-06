mod bin_ops;
mod expr;
mod ident;
mod literals;
mod parsable;
mod statement;
mod syntax_elements;

use chumsky::{prelude::end, Parser};
use parsable::ParserError;
use statement::TopLevelStatement;

pub use parsable::Parsable;

pub fn parser() -> impl Parser<char, Vec<TopLevelStatement>, Error = ParserError> {
    TopLevelStatement::parser().repeated().then_ignore(end())
}
