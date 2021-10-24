use std::fmt::Display;

/// A [`String`]`  ->  `[`String`] key-value pair.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    pub key: String,
    pub value: String,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} {:?}", self.key, self.value))
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_test_data::{test_property_in, test_property_out};

    #[test]
    fn test_property_to_string() {
        assert_eq!(test_property_out().to_string(), test_property_in())
    }
}
