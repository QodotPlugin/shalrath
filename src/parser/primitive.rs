//! [`nom`] functions for parsing Rust primitives.
use nom::{branch::alt, combinator::map_res, IResult};

use super::{parse_integer_unsigned, parse_float, parse_integer_signed};

/// Parse an unsigned decimal literal into a u32
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(parse_integer_unsigned, |out: &str| out.parse::<u32>())(input)
}

/// Parse a floating-point or signed decimal literal into an f32
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    map_res(alt((parse_float, parse_integer_signed)), |out: &str| {
        out.parse::<f32>()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32() {
        assert_eq!(parse_f32("0"), Ok(("", 0.0)));
        assert_eq!(parse_f32("-0"), Ok(("", -0.0)));
        assert_eq!(parse_f32("1.0"), Ok(("", 1.0)));
        assert_eq!(parse_f32("1.000000"), Ok(("", 1.0)));
        assert_eq!(parse_f32("-1.000000"), Ok(("", -1.0)));
        assert_eq!(parse_f32("-1.51788e-17"), Ok(("", -1.51788e-17)));
        assert_eq!(parse_f32("0."), Ok(("", 0.0)));
        assert_eq!(parse_f32(".0"), Ok(("", 0.0)));
    }
}
