#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, ProcessTextError> {
    let original = value.as_ref();

    if original.trim().is_empty() {
        Err(ProcessTextError::Empty)
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
pub enum ProcessTextError {
    Empty,
}

impl fmt::Display for ProcessTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geologic process text cannot be empty"),
        }
    }
}

impl Error for ProcessTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessParseError {
    Empty,
}

impl fmt::Display for ProcessParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geologic process vocabulary cannot be empty"),
        }
    }
}

impl Error for ProcessParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessRateError {
    InvalidFormat,
    InvalidNumber,
    NonFiniteValue,
    EmptyUnit,
}

impl fmt::Display for ProcessRateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => {
                formatter.write_str("process rate must be in '<value> <unit>' format")
            },
            Self::InvalidNumber => formatter.write_str("process rate value must be a valid number"),
            Self::NonFiniteValue => formatter.write_str("process rate value must be finite"),
            Self::EmptyUnit => formatter.write_str("process rate unit cannot be empty"),
        }
    }
}

impl Error for ProcessRateError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeologicProcess(String);

impl GeologicProcess {
    /// Creates a geologic process name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ProcessTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for GeologicProcess {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeologicProcess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeologicProcess {
    type Err = ProcessTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProcessKind {
    Weathering,
    Erosion,
    Deposition,
    Lithification,
    Metamorphism,
    Melting,
    Uplift,
    Subsidence,
    Volcanism,
    Unknown,
    Custom(String),
}

impl fmt::Display for ProcessKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weathering => formatter.write_str("weathering"),
            Self::Erosion => formatter.write_str("erosion"),
            Self::Deposition => formatter.write_str("deposition"),
            Self::Lithification => formatter.write_str("lithification"),
            Self::Metamorphism => formatter.write_str("metamorphism"),
            Self::Melting => formatter.write_str("melting"),
            Self::Uplift => formatter.write_str("uplift"),
            Self::Subsidence => formatter.write_str("subsidence"),
            Self::Volcanism => formatter.write_str("volcanism"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ProcessKind {
    type Err = ProcessParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ProcessParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "weathering" => Ok(Self::Weathering),
            "erosion" => Ok(Self::Erosion),
            "deposition" => Ok(Self::Deposition),
            "lithification" => Ok(Self::Lithification),
            "metamorphism" => Ok(Self::Metamorphism),
            "melting" => Ok(Self::Melting),
            "uplift" => Ok(Self::Uplift),
            "subsidence" => Ok(Self::Subsidence),
            "volcanism" => Ok(Self::Volcanism),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessRate {
    value: f64,
    unit: String,
}

impl ProcessRate {
    /// Creates a process rate from a finite value and non-empty unit.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessRateError::NonFiniteValue`] when the value is not finite.
    /// Returns [`ProcessRateError::EmptyUnit`] when the trimmed unit is empty.
    pub fn new(value: f64, unit: impl AsRef<str>) -> Result<Self, ProcessRateError> {
        if !value.is_finite() {
            return Err(ProcessRateError::NonFiniteValue);
        }

        let unit = unit.as_ref();
        if unit.trim().is_empty() {
            return Err(ProcessRateError::EmptyUnit);
        }

        Ok(Self {
            value,
            unit: unit.to_string(),
        })
    }

    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

impl fmt::Display for ProcessRate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.value, self.unit)
    }
}

impl FromStr for ProcessRate {
    type Err = ProcessRateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        let (numeric_value, unit) = trimmed
            .split_once(char::is_whitespace)
            .ok_or(ProcessRateError::InvalidFormat)?;

        let numeric_value = numeric_value
            .parse::<f64>()
            .map_err(|_| ProcessRateError::InvalidNumber)?;

        Self::new(numeric_value, unit.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GeologicProcess, ProcessKind, ProcessParseError, ProcessRate, ProcessRateError,
        ProcessTextError,
    };

    #[test]
    fn geologic_process_wrapper() -> Result<(), ProcessTextError> {
        let process = GeologicProcess::new("delta progradation")?;

        assert_eq!(process.as_str(), "delta progradation");
        Ok(())
    }

    #[test]
    fn process_kind_display_parse() -> Result<(), ProcessParseError> {
        assert_eq!(ProcessKind::Weathering.to_string(), "weathering");
        assert_eq!("erosion".parse::<ProcessKind>()?, ProcessKind::Erosion);
        Ok(())
    }

    #[test]
    fn custom_process_kind() -> Result<(), ProcessParseError> {
        assert_eq!(
            "diagenesis".parse::<ProcessKind>()?,
            ProcessKind::Custom("diagenesis".to_string())
        );
        Ok(())
    }

    #[test]
    fn process_rate_construction() -> Result<(), ProcessRateError> {
        let rate = ProcessRate::new(0.2, "mm/yr")?;

        assert!((rate.value() - 0.2).abs() < f64::EPSILON);
        assert_eq!(rate.unit(), "mm/yr");
        Ok(())
    }

    #[test]
    fn process_rate_display_is_stable() -> Result<(), ProcessRateError> {
        let rate = "1.5 mm/yr".parse::<ProcessRate>()?;

        assert_eq!(rate.to_string(), "1.5 mm/yr");
        Ok(())
    }
}
