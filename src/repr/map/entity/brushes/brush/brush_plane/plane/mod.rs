mod point;

pub use point::*;

use std::fmt::Display;

/// A plane described by three [`Point`]s.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrianglePlane {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
}

impl Display for TrianglePlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.v0, self.v1, self.v2))
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_test_data::{test_plane_in, test_plane_out};

    #[test]
    fn test_plane_to_string() {
        assert_eq!(test_plane_out().to_string(), test_plane_in())
    }
}
