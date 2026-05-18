#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, GeologicTimeTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(GeologicTimeTextError::Empty)
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
pub enum GeologicTimeTextError {
    Empty,
}

impl fmt::Display for GeologicTimeTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geologic time text cannot be empty"),
        }
    }
}

impl Error for GeologicTimeTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeologicTimeParseError {
    Empty,
}

impl fmt::Display for GeologicTimeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geologic time vocabulary cannot be empty"),
        }
    }
}

impl Error for GeologicTimeParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeologicAgeError {
    InvalidNumber,
    NonFinite,
    Negative,
}

impl fmt::Display for GeologicAgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => formatter.write_str("geologic age must be a valid number"),
            Self::NonFinite => formatter.write_str("geologic age must be finite"),
            Self::Negative => formatter.write_str("geologic age cannot be negative"),
        }
    }
}

impl Error for GeologicAgeError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeologicTimeUnit {
    Eon,
    Era,
    Period,
    Epoch,
    Age,
    Unknown,
    Custom(String),
}

impl fmt::Display for GeologicTimeUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eon => formatter.write_str("eon"),
            Self::Era => formatter.write_str("era"),
            Self::Period => formatter.write_str("period"),
            Self::Epoch => formatter.write_str("epoch"),
            Self::Age => formatter.write_str("age"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for GeologicTimeUnit {
    type Err = GeologicTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(GeologicTimeParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "eon" => Ok(Self::Eon),
            "era" => Ok(Self::Era),
            "period" => Ok(Self::Period),
            "epoch" => Ok(Self::Epoch),
            "age" => Ok(Self::Age),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeologicEon {
    Hadean,
    Archean,
    Proterozoic,
    Phanerozoic,
    Unknown,
    Custom(String),
}

impl fmt::Display for GeologicEon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hadean => formatter.write_str("hadean"),
            Self::Archean => formatter.write_str("archean"),
            Self::Proterozoic => formatter.write_str("proterozoic"),
            Self::Phanerozoic => formatter.write_str("phanerozoic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for GeologicEon {
    type Err = GeologicTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(GeologicTimeParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "hadean" => Ok(Self::Hadean),
            "archean" => Ok(Self::Archean),
            "proterozoic" => Ok(Self::Proterozoic),
            "phanerozoic" => Ok(Self::Phanerozoic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeologicEra {
    Precambrian,
    Paleozoic,
    Mesozoic,
    Cenozoic,
    Unknown,
    Custom(String),
}

impl fmt::Display for GeologicEra {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Precambrian => formatter.write_str("precambrian"),
            Self::Paleozoic => formatter.write_str("paleozoic"),
            Self::Mesozoic => formatter.write_str("mesozoic"),
            Self::Cenozoic => formatter.write_str("cenozoic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for GeologicEra {
    type Err = GeologicTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(GeologicTimeParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "precambrian" => Ok(Self::Precambrian),
            "paleozoic" => Ok(Self::Paleozoic),
            "mesozoic" => Ok(Self::Mesozoic),
            "cenozoic" => Ok(Self::Cenozoic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeologicPeriod {
    Cambrian,
    Ordovician,
    Silurian,
    Devonian,
    Carboniferous,
    Permian,
    Triassic,
    Jurassic,
    Cretaceous,
    Paleogene,
    Neogene,
    Quaternary,
    Unknown,
    Custom(String),
}

impl fmt::Display for GeologicPeriod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cambrian => formatter.write_str("cambrian"),
            Self::Ordovician => formatter.write_str("ordovician"),
            Self::Silurian => formatter.write_str("silurian"),
            Self::Devonian => formatter.write_str("devonian"),
            Self::Carboniferous => formatter.write_str("carboniferous"),
            Self::Permian => formatter.write_str("permian"),
            Self::Triassic => formatter.write_str("triassic"),
            Self::Jurassic => formatter.write_str("jurassic"),
            Self::Cretaceous => formatter.write_str("cretaceous"),
            Self::Paleogene => formatter.write_str("paleogene"),
            Self::Neogene => formatter.write_str("neogene"),
            Self::Quaternary => formatter.write_str("quaternary"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for GeologicPeriod {
    type Err = GeologicTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(GeologicTimeParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "cambrian" => Ok(Self::Cambrian),
            "ordovician" => Ok(Self::Ordovician),
            "silurian" => Ok(Self::Silurian),
            "devonian" => Ok(Self::Devonian),
            "carboniferous" => Ok(Self::Carboniferous),
            "permian" => Ok(Self::Permian),
            "triassic" => Ok(Self::Triassic),
            "jurassic" => Ok(Self::Jurassic),
            "cretaceous" => Ok(Self::Cretaceous),
            "paleogene" => Ok(Self::Paleogene),
            "neogene" => Ok(Self::Neogene),
            "quaternary" => Ok(Self::Quaternary),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeologicEpoch(String);

impl GeologicEpoch {
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeologicTimeTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for GeologicEpoch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeologicEpoch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeologicEpoch {
    type Err = GeologicTimeTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GeologicAge(f64);

impl GeologicAge {
    pub fn new(millions_of_years_before_present: f64) -> Result<Self, GeologicAgeError> {
        if !millions_of_years_before_present.is_finite() {
            return Err(GeologicAgeError::NonFinite);
        }

        if millions_of_years_before_present < 0.0 {
            return Err(GeologicAgeError::Negative);
        }

        Ok(Self(millions_of_years_before_present))
    }

    #[must_use]
    pub fn millions_of_years_before_present(self) -> f64 {
        self.0
    }
}

impl fmt::Display for GeologicAge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for GeologicAge {
    type Err = GeologicAgeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| GeologicAgeError::InvalidNumber)?;
        Self::new(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GeologicAge, GeologicAgeError, GeologicEon, GeologicEpoch, GeologicEra, GeologicPeriod,
        GeologicTimeParseError, GeologicTimeTextError, GeologicTimeUnit,
    };

    #[test]
    fn geologic_time_unit_display_parse() -> Result<(), GeologicTimeParseError> {
        assert_eq!(GeologicTimeUnit::Epoch.to_string(), "epoch");
        assert_eq!(
            "period".parse::<GeologicTimeUnit>()?,
            GeologicTimeUnit::Period
        );
        Ok(())
    }

    #[test]
    fn geologic_eon_display_parse() -> Result<(), GeologicTimeParseError> {
        assert_eq!(GeologicEon::Phanerozoic.to_string(), "phanerozoic");
        assert_eq!("archean".parse::<GeologicEon>()?, GeologicEon::Archean);
        Ok(())
    }

    #[test]
    fn geologic_era_display_parse() -> Result<(), GeologicTimeParseError> {
        assert_eq!(GeologicEra::Mesozoic.to_string(), "mesozoic");
        assert_eq!("cenozoic".parse::<GeologicEra>()?, GeologicEra::Cenozoic);
        Ok(())
    }

    #[test]
    fn geologic_period_display_parse() -> Result<(), GeologicTimeParseError> {
        assert_eq!(GeologicPeriod::Jurassic.to_string(), "jurassic");
        assert_eq!(
            "carboniferous".parse::<GeologicPeriod>()?,
            GeologicPeriod::Carboniferous
        );
        Ok(())
    }

    #[test]
    fn geologic_epoch_wrapper() -> Result<(), GeologicTimeTextError> {
        let epoch = GeologicEpoch::new("Holocene")?;

        assert_eq!(epoch.as_str(), "Holocene");
        Ok(())
    }

    #[test]
    fn valid_geologic_age() -> Result<(), GeologicAgeError> {
        let age = GeologicAge::new(145.0)?;

        assert_eq!(age.millions_of_years_before_present(), 145.0);
        Ok(())
    }

    #[test]
    fn negative_geologic_age_rejected() {
        assert_eq!(GeologicAge::new(-1.0), Err(GeologicAgeError::Negative));
    }
}
