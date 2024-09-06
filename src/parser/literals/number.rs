use chumsky::{
    error::Simple,
    prelude::{choice, just},
    text, Parser,
};

use crate::parser::{
    parsable::{Parsable, ParserError},
    syntax_elements::DotOp,
};

fn based_float_literal_to_value(base: u32, int: &str, fractional: &str) -> f64 {
    let int = u64::from_str_radix(int, base).unwrap() as f64;

    let fractional_len = fractional.len();
    if fractional_len == 0 {
        return int;
    }

    let mut fractional = u64::from_str_radix(fractional, base).unwrap() as f64;

    // shift the number behind its decimal point
    fractional /= base.pow(fractional_len as u32) as f64;
    int + fractional
}

#[test]
fn test_based_float_literal_to_value() {
    assert_eq!(based_float_literal_to_value(10, "123", "321"), 123.321);
    assert_eq!(based_float_literal_to_value(16, "ff", ""), 0xff as f64);
    assert_eq!(based_float_literal_to_value(16, "0f", ""), 0x0f as f64);
    assert_eq!(
        based_float_literal_to_value(16, "0", "f") + based_float_literal_to_value(16, "0", "1"),
        1_f64
    );
}

// TODO allow underscores for readability
#[derive(Debug, PartialEq)]
pub struct NumLit(pub f64);

impl NumLit {
    fn raw_base_parser<const BASE: u32>(
        symbol: &'static str,
    ) -> impl Parser<char, Self, Error = ParserError> {
        just(symbol)
            .ignored()
            // ignore leading zeros
            .then_ignore(just('0').repeated())
            .then(text::int(BASE))
            .then(DotOp::parser().ignored().then(text::int(BASE)).or_not())
            .try_map(|((_, int), fractional), span| {
                if fractional.is_some() && BASE > 10 {
                    // 0x0.dead_beef() is ambiguous
                    return Err(Simple::custom(
                        span,
                        "float literals for bases greater than 10 are not supported.",
                    ));
                }

                // the fractional string doesn't exist -> ""
                let fractional = fractional.unwrap_or(((), "".to_string())).1;

                Ok(Self(based_float_literal_to_value(BASE, &int, &fractional)))
            })
    }
}

impl Parsable for NumLit {
    fn parser() -> impl Parser<char, Self, Error = ParserError>
    where
        Self: Sized,
    {
        choice((
            Self::raw_base_parser::<16>("0x"),
            Self::raw_base_parser::<2>("0b"),
            Self::raw_base_parser::<8>("0o"),
            Self::raw_base_parser::<10>(""),
        ))
    }
}

#[test]
fn test_num_literal() {
    let parse = |str| NumLit::parser().parse(str);

    assert_eq!(parse("1234.4321").unwrap(), NumLit(1234.4321));
    assert_eq!(parse("00743.6400").unwrap(), NumLit(743.64));

    assert_eq!(parse("0b101011.0").unwrap(), NumLit(0b101011 as f64));
    assert_eq!(parse("0xFF3B").unwrap(), NumLit(0xFF3B as f64));
    assert!(parse("0x123.FF").is_err());

    // this looks bad but is fine when parsed as a statement
    assert_eq!(parse("0b1013").unwrap(), NumLit(0b101 as f64));
    assert_eq!(parse("0b101.103").unwrap(), NumLit(0b101 as f64 + (0.5)));
}
