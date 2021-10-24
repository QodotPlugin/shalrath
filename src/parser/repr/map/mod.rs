mod entity;

pub use entity::*;

use std::str::FromStr;

use nom::{
    branch::alt, character::complete::line_ending, combinator::map_res, error::Error,
    multi::separated_list1, Finish, IResult,
};

use crate::{
    repr::{Entity, Map},
    parser::parse_eol_comment,
};

impl FromStr for Map {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_map(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`Map`] from `&str`.
pub fn parse_map(input: &str) -> IResult<&str, Map> {
    let some_entity = map_res(parse_entity, |res| {
        Ok(Some(res)) as Result<Option<Entity>, ()>
    });
    let none_comment = map_res(parse_eol_comment, |_| {
        Ok(None) as Result<Option<Entity>, ()>
    });
    let (i, o) = separated_list1(line_ending, alt((some_entity, none_comment)))(input)?;

    Ok((i, Map::new(o.into_iter().flatten().collect())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_map_in, test_map_out};

    #[test]
    fn test_map() {
        assert_eq!(parse_map(&test_map_in()), Ok(("", test_map_out())));
    }
}
