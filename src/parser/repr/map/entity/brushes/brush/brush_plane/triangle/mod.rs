mod point;

pub use point::*;

use std::str::FromStr;

use nom::{
    character::complete::space1,
    error::Error,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

use crate::repr::Triangle;

impl FromStr for Triangle {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_triangle(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`Triangle`] from `&str`.
pub fn parse_triangle(input: &str) -> IResult<&str, Triangle> {
    let (i, (v0, v1, v2)) = tuple((
        terminated(parse_point, space1),
        parse_point,
        preceded(space1, parse_point),
    ))(input)?;

    Ok((i, Triangle { v0, v1, v2 }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_plane_in, test_plane_out};

    #[test]
    fn test_plane() {
        assert_eq!(parse_triangle(test_plane_in()), Ok(("", test_plane_out())));
    }
}
