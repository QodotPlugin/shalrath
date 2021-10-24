mod brushes;
mod properties;

pub use brushes::*;
pub use properties::*;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::opt,
    error::Error,
    sequence::{delimited, preceded, separated_pair, terminated},
    Finish, IResult,
};

use crate::repr::Entity;

impl FromStr for Entity {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_entity(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse an [`Entity`] from `&str`.
pub fn parse_entity(input: &str) -> IResult<&str, Entity> {
    let (i, o) = delimited(
        terminated(tag("{"), opt(line_ending)),
        separated_pair(parse_properties, opt(line_ending), parse_brushes),
        preceded(opt(line_ending), tag("}")),
    )(input)?;

    Ok((
        i,
        Entity {
            properties: o.0,
            brushes: o.1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_entity_in, test_entity_out};

    #[test]
    fn test_entity() {
        // No properties, brushes
        assert_eq!(
            parse_entity(&("{\n".to_string() + &crate::unit_test_data::test_brush_in() + "\n}")),
            Ok((
                "",
                Entity {
                    properties: Default::default(),
                    brushes: crate::repr::Brushes::new(vec![crate::unit_test_data::test_brush_out()])
                }
            ))
        );

        // Properties, no brushes
        assert_eq!(
            parse_entity("{\n\"classname\" \"worldspawn\"\n}"),
            Ok((
                "",
                Entity {
                    properties: crate::repr::Properties::new(vec![crate::repr::Property {
                        key: "classname".into(),
                        value: "worldspawn".into()
                    }]),
                    brushes: Default::default()
                }
            ))
        );

        // Properties and brushes
        assert_eq!(parse_entity(&test_entity_in()), Ok(("", test_entity_out())));
    }
}
