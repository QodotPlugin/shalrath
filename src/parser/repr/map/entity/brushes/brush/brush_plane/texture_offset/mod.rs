mod texture_plane;

pub use texture_plane::*;

use nom::{
    branch::alt, character::complete::space1, combinator::map_res, sequence::separated_pair,
    IResult,
};

use crate::{repr::TextureOffset, parser::primitive::parse_f32};

/// Parse a [`TextureOffset`] from `&str`
pub fn parse_texture_offset(input: &str) -> IResult<&str, TextureOffset> {
    alt((parse_texture_offset_standard, parse_texture_offset_valve))(input)
}

/// Parse a [`TextureOffset::Standard`] from `&str`
pub fn parse_texture_offset_standard(input: &str) -> IResult<&str, TextureOffset> {
    map_res(separated_pair(parse_f32, space1, parse_f32), |(u, v)| {
        Ok(TextureOffset::Standard { u, v }) as Result<TextureOffset, ()>
    })(input)
}

/// Parse a [`TextureOffset::Valve`] from `&str`
pub fn parse_texture_offset_valve(input: &str) -> IResult<&str, TextureOffset> {
    map_res(
        separated_pair(parse_texture_plane, space1, parse_texture_plane),
        |(u, v)| Ok(TextureOffset::Valve { u, v }) as Result<TextureOffset, ()>,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_texture_offset_in, test_texture_offset_out};

    #[test]
    fn test_valve_texture_offset() {
        assert_eq!(
            parse_texture_offset_valve(test_texture_offset_in()),
            Ok(("", test_texture_offset_out()))
        );
    }
}
