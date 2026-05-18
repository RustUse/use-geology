#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, MineralTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(MineralTextError::Empty)
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
pub enum MineralTextError {
    Empty,
}

impl fmt::Display for MineralTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("mineral text cannot be empty"),
        }
    }
}

impl Error for MineralTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MineralParseError {
    Empty,
}

impl fmt::Display for MineralParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("mineral vocabulary cannot be empty"),
        }
    }
}

impl Error for MineralParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MohsHardnessError {
    InvalidNumber,
    NonFinite,
    OutOfRange,
}

impl fmt::Display for MohsHardnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => formatter.write_str("mohs hardness must be a valid number"),
            Self::NonFinite => formatter.write_str("mohs hardness must be finite"),
            Self::OutOfRange => formatter.write_str("mohs hardness must be in 1.0..=10.0"),
        }
    }
}

impl Error for MohsHardnessError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MineralName(String);

impl MineralName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, MineralTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for MineralName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MineralName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MineralName {
    type Err = MineralTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MineralKind(String);

impl MineralKind {
    pub fn new(value: impl AsRef<str>) -> Result<Self, MineralTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MineralKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MineralKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MineralKind {
    type Err = MineralTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MineralClass {
    Silicate,
    Carbonate,
    Oxide,
    Sulfide,
    Sulfate,
    Halide,
    Phosphate,
    NativeElement,
    Organic,
    Unknown,
    Custom(String),
}

impl fmt::Display for MineralClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Silicate => formatter.write_str("silicate"),
            Self::Carbonate => formatter.write_str("carbonate"),
            Self::Oxide => formatter.write_str("oxide"),
            Self::Sulfide => formatter.write_str("sulfide"),
            Self::Sulfate => formatter.write_str("sulfate"),
            Self::Halide => formatter.write_str("halide"),
            Self::Phosphate => formatter.write_str("phosphate"),
            Self::NativeElement => formatter.write_str("native-element"),
            Self::Organic => formatter.write_str("organic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for MineralClass {
    type Err = MineralParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(MineralParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "silicate" => Ok(Self::Silicate),
            "carbonate" => Ok(Self::Carbonate),
            "oxide" => Ok(Self::Oxide),
            "sulfide" => Ok(Self::Sulfide),
            "sulfate" => Ok(Self::Sulfate),
            "halide" => Ok(Self::Halide),
            "phosphate" => Ok(Self::Phosphate),
            "native-element" => Ok(Self::NativeElement),
            "organic" => Ok(Self::Organic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CrystalSystem {
    Cubic,
    Tetragonal,
    Orthorhombic,
    Hexagonal,
    Trigonal,
    Monoclinic,
    Triclinic,
    Amorphous,
    Unknown,
    Custom(String),
}

impl fmt::Display for CrystalSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cubic => formatter.write_str("cubic"),
            Self::Tetragonal => formatter.write_str("tetragonal"),
            Self::Orthorhombic => formatter.write_str("orthorhombic"),
            Self::Hexagonal => formatter.write_str("hexagonal"),
            Self::Trigonal => formatter.write_str("trigonal"),
            Self::Monoclinic => formatter.write_str("monoclinic"),
            Self::Triclinic => formatter.write_str("triclinic"),
            Self::Amorphous => formatter.write_str("amorphous"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for CrystalSystem {
    type Err = MineralParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(MineralParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "cubic" => Ok(Self::Cubic),
            "tetragonal" => Ok(Self::Tetragonal),
            "orthorhombic" => Ok(Self::Orthorhombic),
            "hexagonal" => Ok(Self::Hexagonal),
            "trigonal" => Ok(Self::Trigonal),
            "monoclinic" => Ok(Self::Monoclinic),
            "triclinic" => Ok(Self::Triclinic),
            "amorphous" => Ok(Self::Amorphous),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MohsHardness(f64);

impl MohsHardness {
    pub fn new(value: f64) -> Result<Self, MohsHardnessError> {
        if !value.is_finite() {
            return Err(MohsHardnessError::NonFinite);
        }

        if !(1.0..=10.0).contains(&value) {
            return Err(MohsHardnessError::OutOfRange);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for MohsHardness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for MohsHardness {
    type Err = MohsHardnessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| MohsHardnessError::InvalidNumber)?;
        Self::new(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CrystalSystem, MineralClass, MineralName, MineralParseError, MineralTextError,
        MohsHardness, MohsHardnessError,
    };

    #[test]
    fn valid_mineral_name() -> Result<(), MineralTextError> {
        let name = MineralName::new("Quartz")?;

        assert_eq!(name.as_str(), "Quartz");
        assert_eq!(name.to_string(), "Quartz");
        Ok(())
    }

    #[test]
    fn empty_mineral_name_rejected() {
        assert_eq!(MineralName::new("   "), Err(MineralTextError::Empty));
    }

    #[test]
    fn mineral_class_display_parse() -> Result<(), MineralParseError> {
        assert_eq!(MineralClass::Carbonate.to_string(), "carbonate");
        assert_eq!(
            "native element".parse::<MineralClass>()?,
            MineralClass::NativeElement
        );
        Ok(())
    }

    #[test]
    fn crystal_system_display_parse() -> Result<(), MineralParseError> {
        assert_eq!(CrystalSystem::Orthorhombic.to_string(), "orthorhombic");
        assert_eq!(
            "hexagonal".parse::<CrystalSystem>()?,
            CrystalSystem::Hexagonal
        );
        Ok(())
    }

    #[test]
    fn valid_mohs_hardness() -> Result<(), MohsHardnessError> {
        let hardness = MohsHardness::new(7.0)?;

        assert_eq!(hardness.value(), 7.0);
        assert_eq!("8.5".parse::<MohsHardness>()?.value(), 8.5);
        Ok(())
    }

    #[test]
    fn invalid_mohs_hardness_rejected() {
        assert_eq!(MohsHardness::new(0.5), Err(MohsHardnessError::OutOfRange));
        assert_eq!(
            MohsHardness::new(f64::NAN),
            Err(MohsHardnessError::NonFinite)
        );
    }
}
