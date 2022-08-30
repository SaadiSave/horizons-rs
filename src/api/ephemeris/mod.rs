use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod common;
pub mod elements;
pub mod vectors;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepSizeUnit {
    Unitless,
    Minutes,
    Hours,
    Days,
    Years,
    Months,
}

impl StepSizeUnit {
    #[allow(clippy::enum_glob_use)]
    fn as_hz_unit(&self) -> &str {
        use StepSizeUnit::*;
        match self {
            Days => "d",
            Hours => "h",
            Minutes => "m",
            Years => "y",
            Months => "mo",
            Unitless => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepSize {
    value: u32,
    unit: StepSizeUnit,
}

impl From<(u32, StepSizeUnit)> for StepSize {
    fn from((value, unit): (u32, StepSizeUnit)) -> Self {
        Self { value, unit }
    }
}

#[allow(clippy::must_use_candidate)]
impl StepSize {
    /// Creates a new [`StepSize`].
    pub fn new(value: u32, unit: StepSizeUnit) -> Self {
        Self { value, unit }
    }
}

impl Display for StepSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}", self.value, self.unit.as_hz_unit())
    }
}

impl Serialize for StepSize {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

#[repr(u8)]
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EphemType {
    #[serde(rename = "O")]
    Observer,
    #[serde(rename = "E")]
    Elements,
    #[serde(rename = "V")]
    Vectors,
}

#[repr(u8)]
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json")]
    Json,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum TimeSpec {
    Bounded {
        step_size: StepSize,
        start_time: DateTime<Utc>,
        stop_time: DateTime<Utc>,
    },
    List(Vec<DateTime<Utc>>),
}

impl TimeSpec {
    pub fn bounded(
        step_size: impl Into<StepSize>,
        start_time: DateTime<Utc>,
        stop_time: DateTime<Utc>,
    ) -> Self {
        Self::Bounded {
            step_size: step_size.into(),
            start_time,
            stop_time,
        }
    }

    pub fn from_list(list: impl IntoIterator<Item = DateTime<Utc>>) -> Self {
        Self::List(list.into_iter().collect())
    }
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutUnits {
    /// km/day
    KM_D,
    /// km/s
    #[default]
    KM_S,
    /// AU/day
    AU_D,
}

impl Display for OutUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            OutUnits::KM_S => f.write_str("km-s"),
            OutUnits::AU_D => f.write_str("au-d"),
            OutUnits::KM_D => f.write_str("km-d"),
        }
    }
}

impl Serialize for OutUnits {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

#[repr(u8)]
#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefPlane {
    #[default]
    #[serde(rename = "E")]
    Ecliptic,
    #[serde(rename = "F")]
    Frame,
    #[serde(rename = "B")]
    BodyEquator,
}

#[repr(u8)]
#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefSystem {
    #[default]
    ICRF,
    B1950,
}

#[cfg(test)]
mod tests {
    use crate::{
        api::ephemeris::{StepSize, StepSizeUnit},
        TestResult,
    };

    #[test]
    fn test_step_size() -> TestResult {
        assert_eq!(
            "STEP_SIZE=6h",
            serde_urlencoded::to_string([("STEP_SIZE", StepSize::new(6, StepSizeUnit::Hours))])?
        );

        Ok(())
    }
}
