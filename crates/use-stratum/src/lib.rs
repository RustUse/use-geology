#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, StratumTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(StratumTextError::Empty)
    } else {
        Ok(original.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut previous_separator = false;

    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() {
            normalized.push(character.to_ascii_lowercase());
            previous_separator = false;
        } else if (character.is_whitespace() || character == '-' || character == '_')
            && !previous_separator
            && !normalized.is_empty()
        {
            normalized.push('-');
            previous_separator = true;
        }
    }

    if normalized.ends_with('-') {
        let _ = normalized.pop();
    }

    normalized
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StratumTextError {
    Empty,
}

impl fmt::Display for StratumTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("stratum text cannot be empty"),
        }
    }
}

impl Error for StratumTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StratumParseError {
    Empty,
}

impl fmt::Display for StratumParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("stratum vocabulary cannot be empty"),
        }
    }
}

impl Error for StratumParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StratumOrderError {
    InvalidNumber,
}

impl fmt::Display for StratumOrderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => formatter.write_str("stratum order must be a valid integer"),
        }
    }
}

impl Error for StratumOrderError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StratumThicknessError {
    InvalidNumber,
    NonFinite,
    Negative,
}

impl fmt::Display for StratumThicknessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => formatter.write_str("stratum thickness must be a valid number"),
            Self::NonFinite => formatter.write_str("stratum thickness must be finite"),
            Self::Negative => formatter.write_str("stratum thickness cannot be negative"),
        }
    }
}

impl Error for StratumThicknessError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StratumName(String);

impl StratumName {
    /// Creates a stratum name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`StratumTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, StratumTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for StratumName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for StratumName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for StratumName {
    type Err = StratumTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StratumKind {
    Bed,
    Layer,
    Seam,
    Lens,
    Horizon,
    Member,
    Unknown,
    Custom(String),
}

impl fmt::Display for StratumKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bed => formatter.write_str("bed"),
            Self::Layer => formatter.write_str("layer"),
            Self::Seam => formatter.write_str("seam"),
            Self::Lens => formatter.write_str("lens"),
            Self::Horizon => formatter.write_str("horizon"),
            Self::Member => formatter.write_str("member"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for StratumKind {
    type Err = StratumParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(StratumParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "bed" => Ok(Self::Bed),
            "layer" => Ok(Self::Layer),
            "seam" => Ok(Self::Seam),
            "lens" => Ok(Self::Lens),
            "horizon" => Ok(Self::Horizon),
            "member" => Ok(Self::Member),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StratumOrder(u32);

impl StratumOrder {
    #[must_use]
    pub const fn new(position: u32) -> Self {
        Self(position)
    }

    #[must_use]
    pub const fn position(self) -> u32 {
        self.0
    }
}

impl fmt::Display for StratumOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for StratumOrder {
    type Err = StratumOrderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u32>()
            .map_err(|_| StratumOrderError::InvalidNumber)?;
        Ok(Self::new(parsed))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct StratumThickness(f64);

impl StratumThickness {
    /// Creates a non-negative stratum thickness in meters.
    ///
    /// # Errors
    ///
    /// Returns [`StratumThicknessError::NonFinite`] when the value is not finite.
    /// Returns [`StratumThicknessError::Negative`] when the value is negative.
    pub fn new(meters: f64) -> Result<Self, StratumThicknessError> {
        if !meters.is_finite() {
            return Err(StratumThicknessError::NonFinite);
        }

        if meters < 0.0 {
            return Err(StratumThicknessError::Negative);
        }

        Ok(Self(meters))
    }

    #[must_use]
    pub const fn meters(self) -> f64 {
        self.0
    }
}

impl fmt::Display for StratumThickness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for StratumThickness {
    type Err = StratumThicknessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| StratumThicknessError::InvalidNumber)?;
        Self::new(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        StratumKind, StratumName, StratumOrder, StratumParseError, StratumTextError,
        StratumThickness, StratumThicknessError,
    };

    #[test]
    fn valid_stratum_name() -> Result<(), StratumTextError> {
        let name = StratumName::new("Brushy Basin")?;

        assert_eq!(name.as_str(), "Brushy Basin");
        Ok(())
    }

    #[test]
    fn empty_stratum_name_rejected() {
        assert_eq!(StratumName::new("\n"), Err(StratumTextError::Empty));
    }

    #[test]
    fn stratum_kind_display_parse() -> Result<(), StratumParseError> {
        assert_eq!(StratumKind::Horizon.to_string(), "horizon");
        assert_eq!("bed".parse::<StratumKind>()?, StratumKind::Bed);
        Ok(())
    }

    #[test]
    fn stratum_order_construction() {
        let order = StratumOrder::new(2);

        assert_eq!(order.position(), 2);
        assert_eq!(order.to_string(), "2");
    }

    #[test]
    fn valid_stratum_thickness() -> Result<(), StratumThicknessError> {
        let thickness = StratumThickness::new(12.5)?;

        assert!((thickness.meters() - 12.5).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn negative_stratum_thickness_rejected() {
        assert_eq!(
            StratumThickness::new(-0.1),
            Err(StratumThicknessError::Negative)
        );
    }
}
