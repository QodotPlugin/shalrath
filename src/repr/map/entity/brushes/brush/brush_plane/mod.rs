mod extension;
mod plane;
mod texture_offset;

use std::fmt::Display;

pub use extension::*;
pub use plane::*;
pub use texture_offset::*;

/// Composes a 3D plane represented by three points, a texture name, and UV data.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BrushPlane {
    pub plane: TrianglePlane,
    pub texture: String,
    pub texture_offset: TextureOffset,
    pub angle: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub extension: Extension,
}

impl Display for BrushPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {} {} {} {}{}{}",
            self.plane,
            self.texture,
            self.texture_offset,
            self.angle,
            self.scale_x,
            self.scale_y,
            if let Extension::Standard = self.extension {
                ""
            } else {
                " "
            },
            self.extension
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_brush_plane_to_string() {
        assert_eq!(
            crate::unit_test_data::test_brush_plane_out().to_string(),
            crate::unit_test_data::test_brush_plane_in()
        )
    }
}
