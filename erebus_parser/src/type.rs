use crate::{parsable::ParsableParser, Expression, Parsable};

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub expr: Expression,
}

impl Type {
    pub fn parser_with<'src>(
        expression_parser: impl ParsableParser<'src, Expression>,
    ) -> impl ParsableParser<'src, Self> {
        expression_parser.map(|expr| Self { expr })
    }
}

impl Parsable for Type {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

#[cfg(test)]
impl Type {
    pub fn test_from_ident(ident: &str) -> Self {
        use crate::{expression::Variable, Path};

        Self {
            expr: Expression::Variable(Variable(Path::test_value([ident]))),
        }
    }
}
