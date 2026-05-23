#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, PlateTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(PlateTextError::Empty)
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
pub enum PlateTextError {
    Empty,
}

impl fmt::Display for PlateTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tectonic plate text cannot be empty"),
        }
    }
}

impl Error for PlateTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlateParseError {
    Empty,
}

impl fmt::Display for PlateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tectonic plate vocabulary cannot be empty"),
        }
    }
}

impl Error for PlateParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TectonicPlateName(String);

impl TectonicPlateName {
    /// Creates a tectonic plate name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PlateTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlateTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TectonicPlateName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TectonicPlateName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TectonicPlateName {
    type Err = PlateTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlateKind {
    Continental,
    Oceanic,
    Microplate,
    Unknown,
    Custom(String),
}

impl fmt::Display for PlateKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continental => formatter.write_str("continental"),
            Self::Oceanic => formatter.write_str("oceanic"),
            Self::Microplate => formatter.write_str("microplate"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PlateKind {
    type Err = PlateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PlateParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "continental" => Ok(Self::Continental),
            "oceanic" => Ok(Self::Oceanic),
            "microplate" => Ok(Self::Microplate),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlateBoundaryKind {
    Convergent,
    Divergent,
    Transform,
    Unknown,
    Custom(String),
}

impl fmt::Display for PlateBoundaryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Convergent => formatter.write_str("convergent"),
            Self::Divergent => formatter.write_str("divergent"),
            Self::Transform => formatter.write_str("transform"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PlateBoundaryKind {
    type Err = PlateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PlateParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "convergent" => Ok(Self::Convergent),
            "divergent" => Ok(Self::Divergent),
            "transform" => Ok(Self::Transform),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlateMotion(String);

impl PlateMotion {
    /// Creates a plate motion descriptor from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PlateTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlateTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PlateMotion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlateMotion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlateMotion {
    type Err = PlateTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PlateBoundaryKind, PlateKind, PlateMotion, PlateParseError, PlateTextError,
        TectonicPlateName,
    };

    #[test]
    fn valid_plate_name() -> Result<(), PlateTextError> {
        let name = TectonicPlateName::new("Pacific Plate")?;

        assert_eq!(name.as_str(), "Pacific Plate");
        Ok(())
    }

    #[test]
    fn empty_plate_name_rejected() {
        assert_eq!(TectonicPlateName::new("   "), Err(PlateTextError::Empty));
    }

    #[test]
    fn plate_kind_display_parse() -> Result<(), PlateParseError> {
        assert_eq!(PlateKind::Continental.to_string(), "continental");
        assert_eq!("oceanic".parse::<PlateKind>()?, PlateKind::Oceanic);
        Ok(())
    }

    #[test]
    fn plate_boundary_kind_display_parse() -> Result<(), PlateParseError> {
        assert_eq!(PlateBoundaryKind::Convergent.to_string(), "convergent");
        assert_eq!(
            "transform".parse::<PlateBoundaryKind>()?,
            PlateBoundaryKind::Transform
        );
        Ok(())
    }

    #[test]
    fn plate_motion_wrapper() -> Result<(), PlateTextError> {
        let motion = PlateMotion::new("northwest")?;

        assert_eq!(motion.as_str(), "northwest");
        Ok(())
    }
}
