mod entity;

pub use entity::*;

use std::{fmt::Display, ops::Deref};

/// A Quake [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) containing one or more [`Entity`]s.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map(Vec<Entity>);

impl Map {
    pub fn new(entities: Vec<Entity>) -> Self {
        entities.into()
    }
}

impl From<Vec<Entity>> for Map {
    fn from(entities: Vec<Entity>) -> Self {
        Map(entities)
    }
}

impl Deref for Map {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Map {
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
    fn test_map_to_string() {
        assert_eq!(
            crate::unit_test_data::test_map_out().to_string(),
            crate::unit_test_data::test_map_in()
        )
    }
}
