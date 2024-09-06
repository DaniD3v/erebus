use chumsky::{error::Simple, Parser};

pub type ParserError = Simple<char>;

pub trait Parsable
where
    Self: Sized,
{
    fn parser() -> impl Parser<char, Self, Error = ParserError>;

    #[cfg(test)]
    fn from_str(str: &str) -> Self
    where
        Self: Sized,
    {
        Self::parser().parse(str).unwrap()
    }
}
