use std::fmt::Display;

/// Enum representing format-specific brush plane extension data.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Extension {
    /// Standard format, no extension.
    Standard,
    /// Hexen 2 format, unknown numeric value.
    Hexen2(f32),
    /// Quake 2 format.
    Quake2 {
        /// Bitmask.
        content_flags: u32,
        /// Bitmask.
        surface_flags: u32,
        /// Face value.
        value: f32,
    },
    Daikatana {
        unknown: (u32, u32, u32),
        color: (u32, u32, u32),
    },
}

impl Default for Extension {
    fn default() -> Self {
        Extension::Standard
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extension::Standard => Ok(()),
            Extension::Hexen2(d) => f.write_fmt(format_args!("{}", d)),
            Extension::Quake2 {
                content_flags,
                surface_flags,
                value,
            } => f.write_fmt(format_args!(
                "{} {} {}",
                content_flags, surface_flags, value
            )),
            Extension::Daikatana {
                unknown: (i, j, k),
                color: (r, g, b),
            } => f.write_fmt(format_args!("{} {} {} {} {} {}", i, j, k, r, g, b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_test_data::{test_extension_in, test_extension_out};

    #[test]
    fn test_extension_to_string() {
        assert_eq!(test_extension_out().to_string(), test_extension_in())
    }
}
