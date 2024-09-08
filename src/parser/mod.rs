mod bin_ops;
mod expr;
mod ident;
mod literals;
mod parsable;
mod statement;
mod syntax_elements;

use chumsky::{IterParser, Parser};
use parsable::ParsableParser;
use statement::TopLevelStatement;

pub use parsable::Parsable;

pub fn parser<'src>() -> impl ParsableParser<'src, Vec<TopLevelStatement>> {
    TopLevelStatement::parser().repeated().collect()
}
