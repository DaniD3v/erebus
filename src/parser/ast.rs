use chumsky::{IterParser, Parser};

use super::{parsable::ParsableParser, statement::TopLevelStatement, Parsable};

#[derive(Debug, PartialEq)]
pub struct Ast {
    statements: Vec<TopLevelStatement>,
}

impl Parsable for Ast {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        TopLevelStatement::parser()
            .repeated()
            .collect()
            .map(|statements| Self { statements })
    }
}
