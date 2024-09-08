use chumsky::{error::Rich, extra, input::Input, Parser as ChumskyParser};

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

pub type ParserError<'src> = Rich<'src, <ParserInput<'src> as Input<'src>>::Token>;
pub type ParserInput<'src> = &'src str;

pub trait Parsable: Sized {
    fn parser<'src>() -> impl ParsableParser<'src, Self>;

    fn parse(input: ParserInput) -> chumsky::ParseResult<Self, ParserError> {
        Self::parser().parse(input)
    }

    #[cfg(test)]
    fn is_err(input: ParserInput) -> bool {
        Self::parse(input).has_errors()
    }
}
