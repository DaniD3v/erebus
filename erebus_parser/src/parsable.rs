use chumsky::{
    error::Rich,
    extra,
    input::{Input, MappedSpan},
    span::SimpleSpan,
    ParseResult, Parser as ChumskyParser,
};

use crate::span::Span;

pub trait ParsableParser<'src, SELF: Sized>:
    ChumskyParser<'src, ParserInput<'src>, SELF, extra::Err<ParserError<'src>>> + Clone
{
}
impl<
        'src,
        T: ChumskyParser<'src, ParserInput<'src>, SELF, extra::Err<ParserError<'src>>> + Clone,
        SELF: Sized,
    > ParsableParser<'src, SELF> for T
{
}

pub type ParserError<'src> = Rich<'src, <ParserInput<'src> as Input<'src>>::Token, ParserSpan>;
type ParserSpan = Span;

pub type ParserInput<'src> = MappedSpan<ParserSpan, &'src str, fn(SimpleSpan) -> ParserSpan>;

pub trait Parsable: Sized {
    fn parser<'src>() -> impl ParsableParser<'src, Self>;

    fn parse(input: &str) -> ParseResult<Self, ParserError<'_>> {
        Self::parser().parse(input.map_span(|simple_span| simple_span.into()))
    }

    #[cfg(test)]
    fn is_err(input: &str) -> bool {
        Self::parse(input).has_errors()
    }
}

impl<T: Parsable> Parsable for Box<T> {
    fn parser<'src>() -> impl ParsableParser<'src, Self> {
        T::parser().map(|e| Box::new(e))
    }
}
