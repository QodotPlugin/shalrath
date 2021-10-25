use nom::{
    branch::alt,
    character::complete::space1,
    combinator::{map_res, opt},
    sequence::tuple,
    IResult,
};

use crate::{
    parser::{primitive::parse_f32, primitive::parse_u32},
    repr::Extension,
};

/// Parse an [`Extension`] from `&str`.
pub fn parse_extension(input: &str) -> IResult<&str, Extension> {
    map_res(
        opt(alt((parse_extension_daikatana, parse_extension_quake_2, parse_extension_hexen_2))),
        |extension| {
            if let Some(extension) = extension {
                Ok(extension) as Result<Extension, ()>
            } else {
                Ok(Extension::Standard)
            }
        },
    )(input)
}

/// Parse an [`Extension::Hexen2`] from `&str`.
pub fn parse_extension_hexen_2(input: &str) -> IResult<&str, Extension> {
    map_res(parse_f32, |f| {
        Ok(Extension::Hexen2(f)) as Result<Extension, ()>
    })(input)
}

/// Parse an [`Extension::Quake2`] from `&str`.
pub fn parse_extension_quake_2(input: &str) -> IResult<&str, Extension> {
    map_res(
        tuple((parse_u32, space1, parse_u32, space1, parse_f32)),
        |(content_flags, _, surface_flags, _, value)| {
            Ok(Extension::Quake2 {
                content_flags,
                surface_flags,
                value,
            }) as Result<Extension, ()>
        },
    )(input)
}

/// Parse an [`Extension::Daikatana`] from `&str`.
pub fn parse_extension_daikatana(input: &str) -> IResult<&str, Extension> {
    map_res(
        tuple((
            parse_u32, space1, parse_u32, space1, parse_u32, space1, parse_u32, space1, parse_u32,
            space1, parse_u32,
        )),
        |(i, _, j, _, k, _, r, _, g, _, b)| {
            Ok(Extension::Daikatana {
                unknown: (i, j, k),
                color: (r, g, b),
            }) as Result<Extension, ()>
        },
    )(input)
}
