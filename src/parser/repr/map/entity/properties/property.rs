use std::str::FromStr;

use nom::{character::complete::space1, error::Error, sequence::separated_pair, Finish, IResult};

use crate::{repr::Property, parser::parse_string};

impl FromStr for Property {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_property(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`Property`] from `&str`.
pub fn parse_property(input: &str) -> IResult<&str, Property> {
    let (i, o) = separated_pair(parse_string, space1, parse_string)(input)?;
    Ok((
        i,
        Property {
            key: o.0.to_string(),
            value: o.1.to_string(),
        },
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_property_in, test_property_out};

    #[test]
    fn test_property() {
        assert_eq!(
            parse_property(test_property_in()),
            Ok(("", test_property_out()))
        );
    }
}
