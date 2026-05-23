#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, RockTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(RockTextError::Empty)
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
pub enum RockTextError {
    Empty,
}

impl fmt::Display for RockTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("rock text cannot be empty"),
        }
    }
}

impl Error for RockTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RockParseError {
    Empty,
}

impl fmt::Display for RockParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("rock vocabulary cannot be empty"),
        }
    }
}

impl Error for RockParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RockCompositionError {
    EmptyLabel,
    EmptyMineralName,
    NoMineralNames,
}

impl fmt::Display for RockCompositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabel => formatter.write_str("rock composition label cannot be empty"),
            Self::EmptyMineralName => {
                formatter.write_str("rock composition mineral names cannot be empty")
            },
            Self::NoMineralNames => {
                formatter.write_str("rock composition requires at least one mineral name")
            },
        }
    }
}

impl Error for RockCompositionError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RockName(String);

impl RockName {
    /// Creates a rock name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RockTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RockTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RockName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RockName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RockName {
    type Err = RockTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RockKind {
    Igneous,
    Sedimentary,
    Metamorphic,
    Unknown,
    Custom(String),
}

impl fmt::Display for RockKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Igneous => formatter.write_str("igneous"),
            Self::Sedimentary => formatter.write_str("sedimentary"),
            Self::Metamorphic => formatter.write_str("metamorphic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for RockKind {
    type Err = RockParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(RockParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "igneous" => Ok(Self::Igneous),
            "sedimentary" => Ok(Self::Sedimentary),
            "metamorphic" => Ok(Self::Metamorphic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RockTexture {
    Clastic,
    Crystalline,
    Glassy,
    Vesicular,
    Foliated,
    NonFoliated,
    Porphyritic,
    FineGrained,
    CoarseGrained,
    Unknown,
    Custom(String),
}

impl fmt::Display for RockTexture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clastic => formatter.write_str("clastic"),
            Self::Crystalline => formatter.write_str("crystalline"),
            Self::Glassy => formatter.write_str("glassy"),
            Self::Vesicular => formatter.write_str("vesicular"),
            Self::Foliated => formatter.write_str("foliated"),
            Self::NonFoliated => formatter.write_str("non-foliated"),
            Self::Porphyritic => formatter.write_str("porphyritic"),
            Self::FineGrained => formatter.write_str("fine-grained"),
            Self::CoarseGrained => formatter.write_str("coarse-grained"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for RockTexture {
    type Err = RockParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(RockParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "clastic" => Ok(Self::Clastic),
            "crystalline" => Ok(Self::Crystalline),
            "glassy" => Ok(Self::Glassy),
            "vesicular" => Ok(Self::Vesicular),
            "foliated" => Ok(Self::Foliated),
            "non-foliated" => Ok(Self::NonFoliated),
            "porphyritic" => Ok(Self::Porphyritic),
            "fine-grained" => Ok(Self::FineGrained),
            "coarse-grained" => Ok(Self::CoarseGrained),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RockComposition {
    label: Option<String>,
    mineral_names: Vec<String>,
}

impl RockComposition {
    /// Creates a rock composition with a non-empty label and no mineral names.
    ///
    /// # Errors
    ///
    /// Returns [`RockTextError::Empty`] when the trimmed label is empty.
    pub fn with_label(label: impl AsRef<str>) -> Result<Self, RockTextError> {
        Ok(Self {
            label: Some(non_empty_text(label)?),
            mineral_names: Vec::new(),
        })
    }

    /// Creates a rock composition from at least one non-empty mineral name.
    ///
    /// # Errors
    ///
    /// Returns [`RockCompositionError::EmptyMineralName`] when any mineral name is empty.
    /// Returns [`RockCompositionError::NoMineralNames`] when no mineral names are supplied.
    pub fn with_mineral_names<I, S>(mineral_names: I) -> Result<Self, RockCompositionError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mineral_names = collect_mineral_names(mineral_names)?;

        Ok(Self {
            label: None,
            mineral_names,
        })
    }

    /// Creates a labeled rock composition from a non-empty label and mineral list.
    ///
    /// # Errors
    ///
    /// Returns [`RockCompositionError::EmptyLabel`] when the label is empty.
    /// Returns [`RockCompositionError::EmptyMineralName`] when any mineral name is empty.
    /// Returns [`RockCompositionError::NoMineralNames`] when no mineral names are supplied.
    pub fn describe<I, S>(
        label: impl AsRef<str>,
        mineral_names: I,
    ) -> Result<Self, RockCompositionError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let label = label.as_ref();
        if label.trim().is_empty() {
            return Err(RockCompositionError::EmptyLabel);
        }

        Ok(Self {
            label: Some(label.to_string()),
            mineral_names: collect_mineral_names(mineral_names)?,
        })
    }

    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    #[must_use]
    pub fn mineral_names(&self) -> &[String] {
        &self.mineral_names
    }
}

impl fmt::Display for RockComposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.label.as_deref(), self.mineral_names.is_empty()) {
            (Some(label), true) => formatter.write_str(label),
            (Some(label), false) => {
                let mineral_names = self.mineral_names.join(", ");
                write!(formatter, "{label} [{mineral_names}]")
            },
            (None, false) => formatter.write_str(&self.mineral_names.join(", ")),
            (None, true) => formatter.write_str("unspecified"),
        }
    }
}

fn collect_mineral_names<I, S>(mineral_names: I) -> Result<Vec<String>, RockCompositionError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mineral_names = mineral_names
        .into_iter()
        .map(|value| {
            let original = value.as_ref();
            if original.trim().is_empty() {
                Err(RockCompositionError::EmptyMineralName)
            } else {
                Ok(original.to_string())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    if mineral_names.is_empty() {
        Err(RockCompositionError::NoMineralNames)
    } else {
        Ok(mineral_names)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RockComposition, RockCompositionError, RockKind, RockName, RockParseError, RockTextError,
        RockTexture,
    };

    #[test]
    fn valid_rock_name() -> Result<(), RockTextError> {
        let name = RockName::new("Basalt")?;

        assert_eq!(name.as_str(), "Basalt");
        Ok(())
    }

    #[test]
    fn empty_rock_name_rejected() {
        assert_eq!(RockName::new("\t"), Err(RockTextError::Empty));
    }

    #[test]
    fn rock_kind_display_parse() -> Result<(), RockParseError> {
        assert_eq!(RockKind::Igneous.to_string(), "igneous");
        assert_eq!("metamorphic".parse::<RockKind>()?, RockKind::Metamorphic);
        Ok(())
    }

    #[test]
    fn rock_texture_display_parse() -> Result<(), RockParseError> {
        assert_eq!(RockTexture::FineGrained.to_string(), "fine-grained");
        assert_eq!(
            "non foliated".parse::<RockTexture>()?,
            RockTexture::NonFoliated
        );
        Ok(())
    }

    #[test]
    fn custom_rock_kind() -> Result<(), RockParseError> {
        assert_eq!(
            "volcaniclastic".parse::<RockKind>()?,
            RockKind::Custom("volcaniclastic".to_string())
        );
        Ok(())
    }

    #[test]
    fn rock_composition_construction() -> Result<(), RockCompositionError> {
        let composition = RockComposition::describe("felsic", ["Quartz", "Feldspar"])?;

        assert_eq!(composition.label(), Some("felsic"));
        assert_eq!(composition.mineral_names(), ["Quartz", "Feldspar"]);
        assert_eq!(composition.to_string(), "felsic [Quartz, Feldspar]");
        Ok(())
    }
}
