#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FossilTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(FossilTextError::Empty)
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
pub enum FossilTextError {
    Empty,
}

impl fmt::Display for FossilTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("fossil text cannot be empty"),
        }
    }
}

impl Error for FossilTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FossilParseError {
    Empty,
}

impl fmt::Display for FossilParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("fossil vocabulary cannot be empty"),
        }
    }
}

impl Error for FossilParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FossilOccurrenceError {
    MissingReference,
    EmptyFormation,
    EmptyTimeLabel,
}

impl fmt::Display for FossilOccurrenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingReference => {
                formatter.write_str("fossil occurrence requires a formation or time label")
            },
            Self::EmptyFormation => {
                formatter.write_str("fossil occurrence formation cannot be empty")
            },
            Self::EmptyTimeLabel => {
                formatter.write_str("fossil occurrence time label cannot be empty")
            },
        }
    }
}

impl Error for FossilOccurrenceError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FossilName(String);

impl FossilName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, FossilTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FossilName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FossilName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FossilName {
    type Err = FossilTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FossilKind {
    BodyFossil,
    TraceFossil,
    Mold,
    Cast,
    Impression,
    Compression,
    Amber,
    Unknown,
    Custom(String),
}

impl fmt::Display for FossilKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BodyFossil => formatter.write_str("body-fossil"),
            Self::TraceFossil => formatter.write_str("trace-fossil"),
            Self::Mold => formatter.write_str("mold"),
            Self::Cast => formatter.write_str("cast"),
            Self::Impression => formatter.write_str("impression"),
            Self::Compression => formatter.write_str("compression"),
            Self::Amber => formatter.write_str("amber"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FossilKind {
    type Err = FossilParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FossilParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "body-fossil" => Ok(Self::BodyFossil),
            "trace-fossil" => Ok(Self::TraceFossil),
            "mold" => Ok(Self::Mold),
            "cast" => Ok(Self::Cast),
            "impression" => Ok(Self::Impression),
            "compression" => Ok(Self::Compression),
            "amber" => Ok(Self::Amber),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FossilPreservation {
    Permineralized,
    Carbonized,
    Replaced,
    Unaltered,
    Compressed,
    Unknown,
    Custom(String),
}

impl fmt::Display for FossilPreservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Permineralized => formatter.write_str("permineralized"),
            Self::Carbonized => formatter.write_str("carbonized"),
            Self::Replaced => formatter.write_str("replaced"),
            Self::Unaltered => formatter.write_str("unaltered"),
            Self::Compressed => formatter.write_str("compressed"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FossilPreservation {
    type Err = FossilParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FossilParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "permineralized" => Ok(Self::Permineralized),
            "carbonized" => Ok(Self::Carbonized),
            "replaced" => Ok(Self::Replaced),
            "unaltered" => Ok(Self::Unaltered),
            "compressed" => Ok(Self::Compressed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FossilOccurrence {
    formation: Option<String>,
    time_label: Option<String>,
}

impl FossilOccurrence {
    pub fn new(
        formation: Option<String>,
        time_label: Option<String>,
    ) -> Result<Self, FossilOccurrenceError> {
        let formation = sanitize_optional_text(formation, FossilOccurrenceError::EmptyFormation)?;
        let time_label = sanitize_optional_text(time_label, FossilOccurrenceError::EmptyTimeLabel)?;

        if formation.is_none() && time_label.is_none() {
            return Err(FossilOccurrenceError::MissingReference);
        }

        Ok(Self {
            formation,
            time_label,
        })
    }

    #[must_use]
    pub fn formation(&self) -> Option<&str> {
        self.formation.as_deref()
    }

    #[must_use]
    pub fn time_label(&self) -> Option<&str> {
        self.time_label.as_deref()
    }
}

impl fmt::Display for FossilOccurrence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.formation.as_deref(), self.time_label.as_deref()) {
            (Some(formation), Some(time_label)) => {
                write!(formatter, "{} @ {}", formation, time_label)
            },
            (Some(formation), None) => formatter.write_str(formation),
            (None, Some(time_label)) => formatter.write_str(time_label),
            (None, None) => formatter.write_str("unspecified"),
        }
    }
}

fn sanitize_optional_text(
    value: Option<String>,
    empty_error: FossilOccurrenceError,
) -> Result<Option<String>, FossilOccurrenceError> {
    match value {
        Some(text) if text.trim().is_empty() => Err(empty_error),
        Some(text) => Ok(Some(text)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FossilKind, FossilName, FossilOccurrence, FossilOccurrenceError, FossilParseError,
        FossilPreservation, FossilTextError,
    };

    #[test]
    fn valid_fossil_name() -> Result<(), FossilTextError> {
        let name = FossilName::new("Trilobite pygidium")?;

        assert_eq!(name.as_str(), "Trilobite pygidium");
        Ok(())
    }

    #[test]
    fn empty_fossil_name_rejected() {
        assert_eq!(FossilName::new(""), Err(FossilTextError::Empty));
    }

    #[test]
    fn fossil_kind_display_parse() -> Result<(), FossilParseError> {
        assert_eq!(FossilKind::TraceFossil.to_string(), "trace-fossil");
        assert_eq!("body fossil".parse::<FossilKind>()?, FossilKind::BodyFossil);
        Ok(())
    }

    #[test]
    fn fossil_preservation_display_parse() -> Result<(), FossilParseError> {
        assert_eq!(
            FossilPreservation::Permineralized.to_string(),
            "permineralized"
        );
        assert_eq!(
            "replaced".parse::<FossilPreservation>()?,
            FossilPreservation::Replaced
        );
        Ok(())
    }

    #[test]
    fn fossil_occurrence_construction() -> Result<(), FossilOccurrenceError> {
        let occurrence = FossilOccurrence::new(
            Some("Morrison Formation".to_string()),
            Some("Late Jurassic".to_string()),
        )?;

        assert_eq!(occurrence.formation(), Some("Morrison Formation"));
        assert_eq!(occurrence.time_label(), Some("Late Jurassic"));
        assert_eq!(occurrence.to_string(), "Morrison Formation @ Late Jurassic");
        Ok(())
    }
}
