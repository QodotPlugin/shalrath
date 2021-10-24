mod brush;

pub use brush::*;

use std::{fmt::Display, ops::Deref};

#[cfg(doc)]
use crate::repr::Entity;
/// The set of brushes inside an [`Entity`].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Brushes(Vec<Brush>);

impl Brushes {
    pub fn new(brushes: Vec<Brush>) -> Self {
        brushes.into()
    }
}

impl From<Vec<Brush>> for Brushes {
    fn from(brushes: Vec<Brush>) -> Self {
        Brushes(brushes)
    }
}

impl Deref for Brushes {
    type Target = Vec<Brush>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Brushes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len().checked_sub(1).unwrap_or_default() {
            writeln!(f, "{}", self[i])?;
        }

        if let Some(last) = self.last() {
            write!(f, "{}", last)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_brushes_to_string() {
        assert_eq!(
            crate::unit_test_data::test_brushes_out().to_string(),
            crate::unit_test_data::test_brushes_in()
        )
    }
}
