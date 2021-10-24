mod brushes;
mod properties;

use std::fmt::Display;

pub use brushes::*;
pub use properties::*;

/// A gameplay object containing [`Properties`] and [`Brushes`].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entity {
    pub properties: Properties,
    pub brushes: Brushes,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "{}", self.properties)?;
        writeln!(f, "{}", self.brushes)?;
        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entity_to_string() {
        assert_eq!(
            crate::unit_test_data::test_entity_out().to_string(),
            crate::unit_test_data::test_entity_in()
        )
    }
}
