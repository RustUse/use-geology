#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FormationTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(FormationTextError::Empty)
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
pub enum FormationTextError {
    Empty,
}

impl fmt::Display for FormationTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("formation text cannot be empty"),
        }
    }
}

impl Error for FormationTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormationParseError {
    Empty,
}

impl fmt::Display for FormationParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("formation vocabulary cannot be empty"),
        }
    }
}

impl Error for FormationParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormationName(String);

impl FormationName {
    /// Creates a formation name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FormationTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FormationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FormationName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FormationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FormationName {
    type Err = FormationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormationKind {
    Formation,
    Group,
    Supergroup,
    Member,
    Bed,
    Unknown,
    Custom(String),
}

impl fmt::Display for FormationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Formation => formatter.write_str("formation"),
            Self::Group => formatter.write_str("group"),
            Self::Supergroup => formatter.write_str("supergroup"),
            Self::Member => formatter.write_str("member"),
            Self::Bed => formatter.write_str("bed"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FormationKind {
    type Err = FormationParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FormationParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "formation" => Ok(Self::Formation),
            "group" => Ok(Self::Group),
            "supergroup" => Ok(Self::Supergroup),
            "member" => Ok(Self::Member),
            "bed" => Ok(Self::Bed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormationMember(String);

impl FormationMember {
    /// Creates a formation member from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FormationTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FormationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FormationMember {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FormationMember {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FormationMember {
    type Err = FormationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormationGroup(String);

impl FormationGroup {
    /// Creates a formation group from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FormationTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FormationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FormationGroup {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FormationGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FormationGroup {
    type Err = FormationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FormationGroup, FormationKind, FormationMember, FormationName, FormationParseError,
        FormationTextError,
    };

    #[test]
    fn valid_formation_name() -> Result<(), FormationTextError> {
        let name = FormationName::new("Morrison Formation")?;

        assert_eq!(name.as_str(), "Morrison Formation");
        Ok(())
    }

    #[test]
    fn empty_formation_name_rejected() {
        assert_eq!(FormationName::new("   "), Err(FormationTextError::Empty));
    }

    #[test]
    fn formation_kind_display_parse() -> Result<(), FormationParseError> {
        assert_eq!(FormationKind::Supergroup.to_string(), "supergroup");
        assert_eq!("group".parse::<FormationKind>()?, FormationKind::Group);
        Ok(())
    }

    #[test]
    fn formation_member_wrapper() -> Result<(), FormationTextError> {
        let member = FormationMember::new("Brushy Basin Member")?;

        assert_eq!(member.as_str(), "Brushy Basin Member");
        Ok(())
    }

    #[test]
    fn formation_group_wrapper() -> Result<(), FormationTextError> {
        let group = FormationGroup::new("Chinle Group")?;

        assert_eq!(group.as_str(), "Chinle Group");
        Ok(())
    }
}
