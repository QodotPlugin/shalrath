//! [`nom`] functions for parsing plaintext [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) data.

pub mod primitive;

#[cfg(doc)]
use crate::repr::Map;

pub mod repr;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{char, none_of, one_of, space1},
    combinator::{opt, recognize},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

/// Recognize an unsigned integer literal.
pub fn parse_integer_unsigned(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

/// Recognize a signed integer literal.
pub fn parse_integer_signed(input: &str) -> IResult<&str, &str> {
    recognize(tuple((opt(one_of("+-")), parse_integer_unsigned)))(input)
}

/// Recognize a floating-point literal.
pub fn parse_float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: +.42 / -.42 / .42
        recognize(tuple((opt(one_of("+-")), char('.'), parse_integer_unsigned))),
        // Case two: 42e42 and 42.42e42
        recognize(tuple((
            parse_integer_signed,
            opt(preceded(char('.'), parse_integer_unsigned)),
            one_of("eE"),
            opt(one_of("+-")),
            parse_integer_unsigned,
        ))),
        // Case two: 42. and 42.42
        recognize(tuple((parse_integer_signed, char('.'), opt(parse_integer_unsigned)))),
    ))(input)
}

/// Parse a quoted string literal into string slice: `"Foo"` becomes an `&str` containing `Foo`.
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    let esc = escaped(none_of("\\\"\'"), '\\', one_of("\"\'"));
    let esc_or_empty = alt((esc, tag("")));
    let res = delimited(one_of("\"\'"), esc_or_empty, one_of("\"\'"))(input)?;

    Ok(res)
}

/// Parse a comment beginning with `//` and terminating at end-of-line, not including end-of-line characters.
pub fn parse_eol_comment(input: &str) -> IResult<&str, &str> {
    let (i, (_, o)) = pair(
        recognize(terminated(tag("//"), opt(space1))),
        is_not("\n\r"),
    )(input)?;
    Ok((i, o))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        assert_eq!(parse_float("1.0"), Ok(("", "1.0")));
        assert_eq!(parse_float("1.000000"), Ok(("", "1.000000")));
        assert_eq!(parse_float("-1.000000"), Ok(("", "-1.000000")));
        assert_eq!(parse_float("0."), Ok(("", "0.")));
        assert_eq!(parse_float(".0"), Ok(("", ".0")));
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_string("\"Foo\""), Ok(("", "Foo")));
        assert_eq!(parse_string("'Foo'"), Ok(("", "Foo")));
    }

    #[test]
    fn test_eol_comment() {
        assert_eq!(parse_eol_comment("// Comment"), Ok(("", "Comment")));
        assert_eq!(parse_eol_comment("// Comment\n"), Ok(("\n", "Comment")));
        assert_eq!(parse_eol_comment("// Comment\r\n"), Ok(("\r\n", "Comment")));
    }
}
