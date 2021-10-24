use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::{opt, recognize},
    error::Error,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

use crate::{repr::Point, parser::primitive::parse_f32};

impl FromStr for Point {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_point(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`Point`] from `&str`.
pub fn parse_point(input: &str) -> IResult<&str, Point> {
    let open_brace = recognize(terminated(tag("("), opt(space1)));
    let triple = tuple((
        terminated(parse_f32, space1),
        parse_f32,
        preceded(space1, parse_f32),
    ));
    let close_brace = recognize(preceded(opt(space1), tag(")")));

    let mut vertex = delimited(open_brace, triple, close_brace);
    let (i, o) = vertex(input)?;
    Ok((
        i,
        Point {
            x: o.0,
            y: o.1,
            z: o.2,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_point_in, test_point_out};

    #[test]
    fn test_vertex() {
        let vertex = test_point_in();
        assert_eq!(parse_point(vertex), Ok(("", test_point_out())));
    }
}
