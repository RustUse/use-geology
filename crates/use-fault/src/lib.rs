#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FaultTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(FaultTextError::Empty)
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
pub enum FaultTextError {
    Empty,
}

impl fmt::Display for FaultTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("fault text cannot be empty"),
        }
    }
}

impl Error for FaultTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultParseError {
    Empty,
}

impl fmt::Display for FaultParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("fault vocabulary cannot be empty"),
        }
    }
}

impl Error for FaultParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FaultName(String);

impl FaultName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, FaultTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FaultName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FaultName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FaultName {
    type Err = FaultTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FaultKind {
    Normal,
    Reverse,
    Thrust,
    StrikeSlip,
    ObliqueSlip,
    Transform,
    Unknown,
    Custom(String),
}

impl fmt::Display for FaultKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => formatter.write_str("normal"),
            Self::Reverse => formatter.write_str("reverse"),
            Self::Thrust => formatter.write_str("thrust"),
            Self::StrikeSlip => formatter.write_str("strike-slip"),
            Self::ObliqueSlip => formatter.write_str("oblique-slip"),
            Self::Transform => formatter.write_str("transform"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FaultKind {
    type Err = FaultParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FaultParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "normal" => Ok(Self::Normal),
            "reverse" => Ok(Self::Reverse),
            "thrust" => Ok(Self::Thrust),
            "strike-slip" => Ok(Self::StrikeSlip),
            "oblique-slip" => Ok(Self::ObliqueSlip),
            "transform" => Ok(Self::Transform),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FaultMovement(String);

impl FaultMovement {
    pub fn new(value: impl AsRef<str>) -> Result<Self, FaultTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FaultMovement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FaultMovement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FaultMovement {
    type Err = FaultTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FaultActivity {
    Active,
    PotentiallyActive,
    Inactive,
    Reactivated,
    Unknown,
    Custom(String),
}

impl fmt::Display for FaultActivity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => formatter.write_str("active"),
            Self::PotentiallyActive => formatter.write_str("potentially-active"),
            Self::Inactive => formatter.write_str("inactive"),
            Self::Reactivated => formatter.write_str("reactivated"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FaultActivity {
    type Err = FaultParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FaultParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "active" => Ok(Self::Active),
            "potentially-active" => Ok(Self::PotentiallyActive),
            "inactive" => Ok(Self::Inactive),
            "reactivated" => Ok(Self::Reactivated),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FaultActivity, FaultKind, FaultMovement, FaultName, FaultParseError, FaultTextError,
    };

    #[test]
    fn valid_fault_name() -> Result<(), FaultTextError> {
        let name = FaultName::new("Wasatch Fault")?;

        assert_eq!(name.as_str(), "Wasatch Fault");
        Ok(())
    }

    #[test]
    fn empty_fault_name_rejected() {
        assert_eq!(FaultName::new("\t"), Err(FaultTextError::Empty));
    }

    #[test]
    fn fault_kind_display_parse() -> Result<(), FaultParseError> {
        assert_eq!(FaultKind::StrikeSlip.to_string(), "strike-slip");
        assert_eq!("reverse".parse::<FaultKind>()?, FaultKind::Reverse);
        Ok(())
    }

    #[test]
    fn fault_movement_wrapper() -> Result<(), FaultTextError> {
        let movement = FaultMovement::new("dextral")?;

        assert_eq!(movement.as_str(), "dextral");
        Ok(())
    }

    #[test]
    fn fault_activity_display_parse() -> Result<(), FaultParseError> {
        assert_eq!(
            FaultActivity::PotentiallyActive.to_string(),
            "potentially-active"
        );
        assert_eq!(
            "inactive".parse::<FaultActivity>()?,
            FaultActivity::Inactive
        );
        Ok(())
    }
}
