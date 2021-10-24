mod texture_plane;

pub use texture_plane::*;

use std::fmt::Display;

/// Format-specific texture offset data.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureOffset {
    /// Standard format, U/V offsets along the closest world texture plane.
    Standard { u: f32, v: f32 },
    /// Valve format, U/V offsets along arbitrary texture planes.
    Valve { u: TexturePlane, v: TexturePlane },
}

impl Default for TextureOffset {
    fn default() -> Self {
        TextureOffset::Standard {
            u: Default::default(),
            v: Default::default(),
        }
    }
}

impl Display for TextureOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureOffset::Standard { u, v } => f.write_fmt(format_args!("{} {}", u, v)),
            TextureOffset::Valve { u, v } => f.write_fmt(format_args!("{} {}", u, v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_test_data::{
        test_texture_offset_in, test_texture_offset_out, test_texture_plane_in,
        test_texture_plane_out,
    };

    #[test]
    fn test_texture_plane_to_string() {
        assert_eq!(
            test_texture_plane_out().to_string(),
            test_texture_plane_in()
        )
    }

    #[test]
    fn test_texture_offset_to_string() {
        assert_eq!(
            test_texture_offset_out().to_string(),
            test_texture_offset_in()
        )
    }
}
