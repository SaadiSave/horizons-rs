#![allow(clippy::module_name_repetitions)]

use crate::request::{
    ephemeris::{EphemType, Format, RefSystem, TimeSpec},
    Center, Command, HzBool,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Common {
    command: Command,
    ephem_type: EphemType,
    center: Center,
    ref_system: RefSystem,
    format: Format,
    obj_data: HzBool,
    make_ephem: HzBool,
    csv_format: HzBool,

    #[serde(flatten)]
    time_spec: TimeSpec,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonBuilderError {
    #[error("Unintialized field `{0}`")]
    UninitializedField(&'static str),
}

#[derive(Debug, Clone)]
pub struct CommonBuilder {
    command: Option<Command>,
    ephem_type: Option<EphemType>,
    center: Option<Center>,
    ref_system: RefSystem,
    time_spec: Option<TimeSpec>,
    format: Format,
    obj_data: bool,
    make_ephem: bool,
    csv_format: bool,
}

impl CommonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(&mut self, command: impl Into<Command>) -> &mut Self {
        self.command = Some(command.into());
        self
    }

    pub fn ephem_type(&mut self, ephem_type: EphemType) -> &mut Self {
        self.ephem_type = Some(ephem_type);
        self
    }

    pub fn center(&mut self, center: impl Into<Center>) -> &mut Self {
        self.center = Some(center.into());
        self
    }

    pub fn ref_system(&mut self, ref_system: RefSystem) -> &mut Self {
        self.ref_system = ref_system;
        self
    }

    pub fn time_spec(&mut self, time_spec: TimeSpec) -> &mut Self {
        self.time_spec = Some(time_spec);
        self
    }

    pub fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    pub fn obj_data(&mut self, obj_data: bool) -> &mut Self {
        self.obj_data = obj_data;
        self
    }

    pub fn make_ephem(&mut self, make_ephem: bool) -> &mut Self {
        self.make_ephem = make_ephem;
        self
    }

    pub fn csv_format(&mut self, csv_format: bool) -> &mut Self {
        self.csv_format = csv_format;
        self
    }

    pub fn build(&self) -> Result<Common, CommonBuilderError> {
        let command = self
            .command
            .clone()
            .ok_or(CommonBuilderError::UninitializedField("command"))?;

        let ephem_type = self
            .ephem_type
            .ok_or(CommonBuilderError::UninitializedField("ephem_type"))?;

        let center = self
            .center
            .clone()
            .ok_or(CommonBuilderError::UninitializedField("center"))?;

        let time_spec = self
            .time_spec
            .clone()
            .ok_or(CommonBuilderError::UninitializedField("time_spec"))?;

        let &Self {
            ref_system,
            format,
            obj_data,
            make_ephem,
            csv_format,
            ..
        } = self;

        Ok(Common {
            command,
            ephem_type,
            center,
            ref_system,
            time_spec,
            format,
            obj_data: obj_data.into(),
            make_ephem: make_ephem.into(),
            csv_format: csv_format.into(),
        })
    }
}

impl Default for CommonBuilder {
    fn default() -> Self {
        Self {
            command: None,
            ephem_type: None,
            center: None,
            ref_system: RefSystem::default(),
            time_spec: None,
            format: Format::Text,
            obj_data: true,
            make_ephem: true,
            csv_format: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        request::{
            bodies::MajorBody,
            ephemeris::{
                common::{Common, CommonBuilder},
                EphemType, Format, RefSystem, StepSizeUnit, TimeSpec,
            },
        },
        TestResult,
    };
    use chrono::Utc;

    #[test]
    fn test_common_builder() -> TestResult {
        let now = Utc::now();

        let case = CommonBuilder::new()
            .command(MajorBody::Europa)
            .ephem_type(EphemType::Vectors)
            .center(MajorBody::Jupiter)
            .time_spec(TimeSpec::bounded(
                (6, StepSizeUnit::Hours),
                now,
                now + chrono::Duration::days(2),
            ))
            .csv_format(true)
            .obj_data(false)
            .build()?;

        assert_eq!(
            case,
            Common {
                command: MajorBody::Europa.into(),
                ephem_type: EphemType::Vectors,
                center: MajorBody::Jupiter.into(),
                time_spec: TimeSpec::Bounded {
                    step_size: (6, StepSizeUnit::Hours).into(),
                    start_time: now,
                    stop_time: now + chrono::Duration::days(2),
                },
                ref_system: RefSystem::ICRF,
                format: Format::Text,
                obj_data: false.into(),
                make_ephem: true.into(),
                csv_format: true.into(),
            }
        );

        println!("{}", serde_urlencoded::to_string(case)?);

        Ok(())
    }
}
