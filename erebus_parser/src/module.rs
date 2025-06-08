use chumsky::{IterParser, Parser};

use super::{parsable::ParsableParser, statement::TopLevelStatement, Parsable};

#[derive(Debug, PartialEq)]
pub struct RootModule {
    module: ModuleContent,
}

impl RootModule {
    pub fn iter_statements(self) -> impl Iterator<Item = TopLevelStatement> {
        self.module.statements.into_iter()
    }
}

impl Parsable for RootModule {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        ModuleContent::parser().map(|module| Self { module })
    }
}

// TODO The reason this is seperate from Root module is because I
// still need to handle the mod keyword.
#[derive(Debug, PartialEq)]
pub struct ModuleContent {
    statements: Vec<TopLevelStatement>,
}

impl Parsable for ModuleContent {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        TopLevelStatement::parser()
            .repeated()
            .collect()
            .map(|statements| Self { statements })
    }
}
