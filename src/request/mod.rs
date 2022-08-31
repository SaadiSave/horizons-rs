#![allow(clippy::must_use_candidate, clippy::missing_errors_doc)]

pub mod bodies;
pub mod ephemeris;
pub mod presets;

use std::fmt::{Display, Formatter, Result as FmtResult};

use bodies::MajorBody;
use ephemeris::{
    common::{Common, CommonBuilder, CommonBuilderError},
    elements::{Elements, ElementsBuilder},
    vectors::{Vectors, VectorsBuilder},
    EphemType,
};
use serde::Serialize;
use thiserror::Error;

#[repr(u8)]
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HzBool {
    #[serde(rename = "yes")]
    Yes = 1,
    #[serde(rename = "no")]
    No = 0,
}

impl From<bool> for HzBool {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Command {
    Body(Body),
    /// Only use this variant if you are absolutely sure about what you are doing
    Custom(String),
}

impl<B: Into<Body>> From<B> for Command {
    fn from(b: B) -> Self {
        Self::Body(b.into())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Body {
    MajorBody(MajorBody),
    /// Only use this variant if you are absolutely sure about what you are doing
    Custom(String),
}

impl From<MajorBody> for Body {
    fn from(b: MajorBody) -> Self {
        Body::MajorBody(b)
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Body::MajorBody(b) => write!(f, "{}", u32::from(b)),
            Body::Custom(s) => f.write_str(s),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Site {
    #[default]
    Center,
    Custom(u16),
}

impl From<u16> for Site {
    fn from(num: u16) -> Self {
        Site::Custom(num)
    }
}

impl Display for Site {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Site::Center => f.write_str("500"),
            Site::Custom(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Center {
    site: Site,
    body: Body,
}

impl<B: Into<Body>> From<B> for Center {
    fn from(body: B) -> Self {
        Self {
            site: Site::Center,
            body: body.into(),
        }
    }
}

impl<S: Into<Site>, B: Into<Body>> From<(S, B)> for Center {
    fn from((site, body): (S, B)) -> Self {
        Self {
            site: site.into(),
            body: body.into(),
        }
    }
}

impl Serialize for Center {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&format!("{}@{}", self.site, self.body))
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Ephemeris {
    // TODO: Observer,
    Elements(Elements),
    Vectors(Vectors),
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Query {
    #[serde(flatten)]
    common: Common,
    #[serde(flatten)]
    specific: Ephemeris,
}

impl Query {
    /// # Example
    /// ```ignore
    /// let mut builder = Query::elements();
    ///
    /// builder.common
    ///     .command(MajorBody::Jupiter)
    ///     .center(MajorBody::SolarSystemBary)
    ///     /* Continue */;
    ///
    /// builder.specific
    ///     .elm_labels(false)
    ///     /* Continue */;
    ///
    /// let query = builder.build()?;
    /// ```
    pub fn elements() -> QueryBuilder<ElementsBuilder> {
        QueryBuilder {
            common: CommonBuilder::new().ephem_type(EphemType::Elements).clone(),
            specific: ElementsBuilder::default(),
        }
    }

    /// # Example
    /// ```ignore
    /// let mut builder = Query::vectors();
    ///
    /// builder.common
    ///     .command(MajorBody::Jupiter)
    ///     .center(MajorBody::SolarSystemBary)
    ///     /* Continue */;
    ///
    /// builder.specific
    ///     .vec_labels(false)
    ///     /* Continue */;
    ///
    /// let query = builder.build()?;
    /// ```
    pub fn vectors() -> QueryBuilder<VectorsBuilder> {
        QueryBuilder {
            common: CommonBuilder::new().ephem_type(EphemType::Vectors).clone(),
            specific: VectorsBuilder::default(),
        }
    }
}

/// Do not use this struct directly. Use one of the functions on [`Query`] instead.
pub struct QueryBuilder<T> {
    pub common: CommonBuilder,
    pub specific: T,
}

#[derive(Error, Debug, Clone)]
pub enum QueryBuilderError {
    #[error("{0}")]
    CommonBuilderError(CommonBuilderError),
}

crate::impl_from_for_inner_enum!(QueryBuilderError: CommonBuilderError);

impl QueryBuilder<ElementsBuilder> {
    pub fn build(&self) -> Result<Query, QueryBuilderError> {
        Ok(Query {
            common: self.common.build()?,
            specific: Ephemeris::Elements(self.specific.build()),
        })
    }
}

impl QueryBuilder<VectorsBuilder> {
    pub fn build(&self) -> Result<Query, QueryBuilderError> {
        Ok(Query {
            common: self.common.build()?,
            specific: Ephemeris::Vectors(self.specific.build()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        request::{
            bodies::{InvalidBodyCode, MajorBody},
            ephemeris::{vectors::Correction, StepSizeUnit, TimeSpec},
            HzBool, Query,
        },
        TestResult,
    };
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn test_hz_bool() -> TestResult {
        let (yes, no): (HzBool, HzBool) = (true.into(), false.into());

        assert_eq!(
            "MAKE=yes&JSON=no",
            serde_urlencoded::to_string([("MAKE", yes), ("JSON", no)])?
        );

        Ok(())
    }

    #[test]
    fn test_major_body() -> TestResult {
        assert_eq!(
            "command=599",
            serde_urlencoded::to_string([("command", MajorBody::Jupiter)])?
        );

        assert_eq!(Ok(MajorBody::JupiterBary), MajorBody::try_from(5));

        assert_eq!(Err(InvalidBodyCode(598)), MajorBody::try_from(598));

        Ok(())
    }

    #[test]
    fn test_query() -> TestResult {
        let mut builder = Query::vectors();
        let start = Utc.ymd(2022, 8, 28).and_hms(0, 0, 0);

        builder
            .common
            .command(MajorBody::Jupiter)
            .center(MajorBody::SolarSystemBary)
            .time_spec(TimeSpec::bounded(
                (6, StepSizeUnit::Hours),
                start,
                start + Duration::days(2),
            ))
            .obj_data(false)
            .csv_format(false);

        builder.specific.vec_corr(Correction::LT_S);

        let query = builder.build()?;

        assert_eq!(
            "command=599&ephem_type=V&center=500%400&ref_system=ICRF&format=text\
            &obj_data=no&make_ephem=yes&csv_format=no&step_size=6h\
            &start_time=2022-08-28T00%3A00%3A00Z&stop_time=2022-08-30T00%3A00%3A00Z\
            &vec_table=3&vec_labels=yes&vec_delta_t=no&vec_corr=LT%2BS&out_units=km-s&ref_plane=E",
            serde_urlencoded::to_string(query)?
        );

        Ok(())
    }
}
