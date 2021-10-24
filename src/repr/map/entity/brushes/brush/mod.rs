mod brush_plane;

pub use brush_plane::*;

use std::{fmt::Display, ops::{Deref, DerefMut}};

/// The convex volume represented by a set of [`BrushPlane`]s.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Brush(Vec<BrushPlane>);

impl Brush {
    pub fn new(entities: Vec<BrushPlane>) -> Self {
        entities.into()
    }
}

impl From<Vec<BrushPlane>> for Brush {
    fn from(entities: Vec<BrushPlane>) -> Self {
        Brush(entities)
    }
}

impl Deref for Brush {
    type Target = Vec<BrushPlane>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Brush {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Brush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for brush in &self.0 {
            f.write_fmt(format_args!("{}\n", brush))?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_brush_to_string() {
        assert_eq!(
            crate::unit_test_data::test_brush_out().to_string(),
            crate::unit_test_data::test_brush_in()
        )
    }
}
