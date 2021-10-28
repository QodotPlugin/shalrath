use std::fmt::Display;

/// A plane described by a normal and distance.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexturePlane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub d: f32,
}

impl Display for TexturePlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[ {} {} {} {} ]",
            self.x, self.y, self.z, self.d
        ))
    }
}
