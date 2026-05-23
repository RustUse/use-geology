#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, SedimentTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(SedimentTextError::Empty)
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
pub enum SedimentTextError {
    Empty,
}

impl fmt::Display for SedimentTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("sediment text cannot be empty"),
        }
    }
}

impl Error for SedimentTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SedimentParseError {
    Empty,
}

impl fmt::Display for SedimentParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("sediment vocabulary cannot be empty"),
        }
    }
}

impl Error for SedimentParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrainSizeError {
    InvalidNumber,
    NonFinite,
    Negative,
}

impl fmt::Display for GrainSizeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => formatter.write_str("grain size must be a valid number"),
            Self::NonFinite => formatter.write_str("grain size must be finite"),
            Self::Negative => formatter.write_str("grain size cannot be negative"),
        }
    }
}

impl Error for GrainSizeError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SedimentName(String);

impl SedimentName {
    /// Creates a sediment name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SedimentTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SedimentTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SedimentName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SedimentName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SedimentName {
    type Err = SedimentTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SedimentKind {
    Clay,
    Silt,
    Sand,
    Gravel,
    Pebble,
    Cobble,
    Boulder,
    Mud,
    Unknown,
    Custom(String),
}

impl fmt::Display for SedimentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clay => formatter.write_str("clay"),
            Self::Silt => formatter.write_str("silt"),
            Self::Sand => formatter.write_str("sand"),
            Self::Gravel => formatter.write_str("gravel"),
            Self::Pebble => formatter.write_str("pebble"),
            Self::Cobble => formatter.write_str("cobble"),
            Self::Boulder => formatter.write_str("boulder"),
            Self::Mud => formatter.write_str("mud"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for SedimentKind {
    type Err = SedimentParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SedimentParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "clay" => Ok(Self::Clay),
            "silt" => Ok(Self::Silt),
            "sand" => Ok(Self::Sand),
            "gravel" => Ok(Self::Gravel),
            "pebble" => Ok(Self::Pebble),
            "cobble" => Ok(Self::Cobble),
            "boulder" => Ok(Self::Boulder),
            "mud" => Ok(Self::Mud),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GrainSize(f64);

impl GrainSize {
    /// Creates a non-negative grain size in millimeters.
    ///
    /// # Errors
    ///
    /// Returns [`GrainSizeError::NonFinite`] when the value is not finite.
    /// Returns [`GrainSizeError::Negative`] when the value is negative.
    pub fn new(millimeters: f64) -> Result<Self, GrainSizeError> {
        if !millimeters.is_finite() {
            return Err(GrainSizeError::NonFinite);
        }

        if millimeters < 0.0 {
            return Err(GrainSizeError::Negative);
        }

        Ok(Self(millimeters))
    }

    #[must_use]
    pub const fn millimeters(self) -> f64 {
        self.0
    }
}

impl fmt::Display for GrainSize {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for GrainSize {
    type Err = GrainSizeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| GrainSizeError::InvalidNumber)?;
        Self::new(parsed)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Sorting {
    WellSorted,
    ModeratelySorted,
    PoorlySorted,
    Unknown,
    Custom(String),
}

impl fmt::Display for Sorting {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WellSorted => formatter.write_str("well-sorted"),
            Self::ModeratelySorted => formatter.write_str("moderately-sorted"),
            Self::PoorlySorted => formatter.write_str("poorly-sorted"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for Sorting {
    type Err = SedimentParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SedimentParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "well-sorted" => Ok(Self::WellSorted),
            "moderately-sorted" => Ok(Self::ModeratelySorted),
            "poorly-sorted" => Ok(Self::PoorlySorted),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Roundness {
    Angular,
    SubAngular,
    SubRounded,
    Rounded,
    WellRounded,
    Unknown,
    Custom(String),
}

impl fmt::Display for Roundness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Angular => formatter.write_str("angular"),
            Self::SubAngular => formatter.write_str("sub-angular"),
            Self::SubRounded => formatter.write_str("sub-rounded"),
            Self::Rounded => formatter.write_str("rounded"),
            Self::WellRounded => formatter.write_str("well-rounded"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for Roundness {
    type Err = SedimentParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SedimentParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "angular" => Ok(Self::Angular),
            "sub-angular" => Ok(Self::SubAngular),
            "sub-rounded" => Ok(Self::SubRounded),
            "rounded" => Ok(Self::Rounded),
            "well-rounded" => Ok(Self::WellRounded),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GrainSize, GrainSizeError, Roundness, SedimentKind, SedimentName, SedimentParseError,
        SedimentTextError, Sorting,
    };

    #[test]
    fn valid_sediment_name() -> Result<(), SedimentTextError> {
        let name = SedimentName::new("Alluvial sand")?;

        assert_eq!(name.as_str(), "Alluvial sand");
        Ok(())
    }

    #[test]
    fn empty_sediment_name_rejected() {
        assert_eq!(SedimentName::new("  "), Err(SedimentTextError::Empty));
    }

    #[test]
    fn sediment_kind_display_parse() -> Result<(), SedimentParseError> {
        assert_eq!(SedimentKind::Gravel.to_string(), "gravel");
        assert_eq!("mud".parse::<SedimentKind>()?, SedimentKind::Mud);
        Ok(())
    }

    #[test]
    fn valid_grain_size() -> Result<(), GrainSizeError> {
        let grain_size = GrainSize::new(0.0625)?;

        assert!((grain_size.millimeters() - 0.0625).abs() < f64::EPSILON);
        assert!(("2.0".parse::<GrainSize>()?.millimeters() - 2.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn negative_grain_size_rejected() {
        assert_eq!(GrainSize::new(-1.0), Err(GrainSizeError::Negative));
    }

    #[test]
    fn sorting_display_parse() -> Result<(), SedimentParseError> {
        assert_eq!(Sorting::WellSorted.to_string(), "well-sorted");
        assert_eq!(
            "moderately sorted".parse::<Sorting>()?,
            Sorting::ModeratelySorted
        );
        Ok(())
    }

    #[test]
    fn roundness_display_parse() -> Result<(), SedimentParseError> {
        assert_eq!(Roundness::WellRounded.to_string(), "well-rounded");
        assert_eq!("sub rounded".parse::<Roundness>()?, Roundness::SubRounded);
        Ok(())
    }
}
