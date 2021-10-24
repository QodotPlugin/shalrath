use std::fmt::Display;

/// A single-precision floating point vector with three components.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "( {}{} {}{} {}{} )",
            if self.x == 0.0 && self.x.is_sign_negative() { "-" } else { "" },
            if self.x == 0.0 { self.x.abs() } else { self.x },
            if self.y == 0.0 && self.y.is_sign_negative() { "-" } else { "" },
            if self.y == 0.0 { self.y.abs() } else { self.y },
            if self.z == 0.0 && self.z.is_sign_negative() { "-" } else { "" },
            if self.z == 0.0 { self.z.abs() } else { self.z },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_test_data::{test_point_in, test_point_out};

    #[test]
    fn test_point_to_string() {
        assert_eq!(test_point_out().to_string(), test_point_in())
    }
}
