mod property;

pub use property::*;

use std::{fmt::Display, ops::{Deref, DerefMut}};

/// A set of [`Property`]s.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Properties(pub Vec<Property>);

impl Properties {
    pub fn new(properties: Vec<Property>) -> Self {
        properties.into()
    }
}

impl From<Vec<Property>> for Properties {
    fn from(properties: Vec<Property>) -> Self {
        Properties(properties)
    }
}

impl Deref for Properties {
    type Target = Vec<Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Properties {
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
    use crate::unit_test_data::{test_properties_in, test_properties_out};

    #[test]
    fn test_properties() {
        assert_eq!(test_properties_out().to_string(), test_properties_in())
    }
}
