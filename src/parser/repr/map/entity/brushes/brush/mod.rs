mod brush_plane;

pub use brush_plane::*;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::recognize,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    Finish, IResult,
};

use crate::repr::Brush;

impl FromStr for Brush {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_brush(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`Brush`] from `&str`.
pub fn parse_brush(input: &str) -> IResult<&str, Brush> {
    let (i, o) = delimited(
        recognize(terminated(tag("{"), line_ending)),
        separated_list1(line_ending, parse_brush_plane),
        preceded(line_ending, tag("}")),
    )(input)?;

    Ok((i, Brush::new(o)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_brush_in, test_brush_out};

    #[test]
    fn test_brush() {
        assert_eq!(parse_brush(&test_brush_in()), Ok(("", test_brush_out())));
    }
}
