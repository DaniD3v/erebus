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
    fn parser<'src>() -> impl crate::parsable::ParsableParser<'src, Self> {
        Self::parser_with(Expression::parser())
    }
}

#[cfg(test)]
impl Type {
    pub fn test_ident(ident: &str) -> Self {
        use crate::{expression::Variable, ident::Ident};

        Self {
            expr: Expression::Variable(Variable(Ident::test_value(ident))),
        }
    }
}
