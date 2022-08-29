#![allow(non_camel_case_types, clippy::module_name_repetitions)]

use crate::api::{
    ephemeris::{OutUnits, RefPlane},
    HzBool,
};
use serde::Serialize;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableFormat {
    /// Position components {x,y,z} only (with optional statistical request codes)
    Position = 1,
    /// State vector {x,y,z,Vx,Vy,Vz} (with optional statistical request codes)
    State = 2,
    /// State vector, 1-way light-time, range, and range-rate
    #[default]
    State_LT = 3,
    /// Position, 1-way light-time, range, and range-rate
    Position_LT = 4,
    /// Velocity components {vx, vy, vz} only
    Velocity = 5,
    /// 1-way light-time, range, and range-rate
    LT = 6,
}

crate::impl_from_int_for_enum!(u8, TableFormat);

impl Serialize for TableFormat {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u8(u8::from(self))
    }
}

#[repr(u8)]
#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correction {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "LT+S")]
    LT_S,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Vectors {
    vec_labels: HzBool,
    vec_delta_t: HzBool,
    vec_corr: Correction,
    out_units: OutUnits,
    ref_plane: RefPlane,
}

#[derive(Debug, Clone, Copy)]
pub struct VectorsBuilder {
    vec_labels: bool,
    vec_delta_t: bool,
    vec_corr: Correction,
    out_units: OutUnits,
    ref_plane: RefPlane,
}

impl VectorsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vec_labels(&mut self, vec_labels: bool) -> &mut Self {
        self.vec_labels = vec_labels;
        self
    }

    pub fn vec_delta_t(&mut self, vec_delta_t: bool) -> &mut Self {
        self.vec_delta_t = vec_delta_t;
        self
    }

    pub fn vec_corr(&mut self, vec_corr: Correction) -> &mut Self {
        self.vec_corr = vec_corr;
        self
    }

    pub fn out_units(&mut self, out_units: OutUnits) -> &mut Self {
        self.out_units = out_units;
        self
    }

    pub fn ref_plane(&mut self, ref_plane: RefPlane) -> &mut Self {
        self.ref_plane = ref_plane;
        self
    }

    pub fn build(&self) -> Vectors {
        let &Self {
            vec_labels,
            vec_delta_t,
            vec_corr,
            out_units,
            ref_plane,
        } = self;

        Vectors {
            vec_labels: vec_labels.into(),
            vec_delta_t: vec_delta_t.into(),
            vec_corr,
            out_units,
            ref_plane,
        }
    }
}

impl Default for VectorsBuilder {
    fn default() -> Self {
        Self {
            vec_labels: true,
            vec_delta_t: false,
            vec_corr: Correction::default(),
            out_units: OutUnits::default(),
            ref_plane: RefPlane::default(),
        }
    }
}
