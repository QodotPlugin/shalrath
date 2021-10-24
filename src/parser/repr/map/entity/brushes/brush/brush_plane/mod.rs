mod extension;
mod texture_offset;
mod triangle;

pub use extension::*;
pub use texture_offset::*;
pub use triangle::*;

use std::str::FromStr;

use nom::{
    bytes::complete::take_until, character::complete::space1, combinator::opt, error::Error,
    sequence::terminated, Finish, IResult,
};

use crate::{repr::BrushPlane, parser::primitive::parse_f32};

impl FromStr for BrushPlane {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_brush_plane(s).finish() {
            Ok((_, o)) => Ok(o),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

/// Parse a [`BrushPlane`] from `&str`
pub fn parse_brush_plane(i: &str) -> IResult<&str, BrushPlane> {
    let (i, plane) = terminated(parse_triangle, space1)(i)?;
    let (i, texture) = terminated(take_until(" "), space1)(i)?;
    let (i, texture_offset) = terminated(parse_texture_offset, space1)(i)?;
    let (i, angle) = terminated(parse_f32, space1)(i)?;
    let (i, scale_x) = terminated(parse_f32, space1)(i)?;
    let (i, scale_y) = terminated(parse_f32, opt(space1))(i)?;
    let (i, extension) = parse_extension(i)?;

    Ok((
        i,
        BrushPlane {
            plane,
            texture: texture.to_string(),
            texture_offset,
            angle,
            scale_x,
            scale_y,
            extension,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_data::{test_brush_plane_in, test_brush_plane_out};

    #[test]
    fn test_brush_plane() {
        assert_eq!(
            parse_brush_plane(&test_brush_plane_in()),
            Ok(("", test_brush_plane_out()))
        );
    }
}
