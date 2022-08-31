#![allow(clippy::module_name_repetitions)]

use crate::request::{
    ephemeris::{OutUnits, RefPlane},
    HzBool,
};
use serde::Serialize;

/// Determines what type of periapsis time (Tp) is returned
#[repr(u8)]
#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpType {
    #[default]
    Absolute,
    Relative,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Elements {
    tp_type: TpType,
    out_units: OutUnits,
    ref_plane: RefPlane,
    elm_labels: HzBool,
}

#[derive(Debug, Clone, Copy)]
pub struct ElementsBuilder {
    tp_type: TpType,
    out_units: OutUnits,
    ref_plane: RefPlane,
    elm_labels: bool,
}

impl ElementsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tp_type(&mut self, tp_type: TpType) -> &mut Self {
        self.tp_type = tp_type;
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

    pub fn elm_labels(&mut self, elm_labels: bool) -> &mut Self {
        self.elm_labels = elm_labels;
        self
    }

    pub fn build(&self) -> Elements {
        let &Self {
            tp_type,
            out_units,
            ref_plane,
            elm_labels,
        } = self;

        Elements {
            tp_type,
            out_units,
            ref_plane,
            elm_labels: elm_labels.into(),
        }
    }
}

impl Default for ElementsBuilder {
    fn default() -> Self {
        Self {
            tp_type: TpType::default(),
            out_units: OutUnits::default(),
            ref_plane: RefPlane::default(),
            elm_labels: true,
        }
    }
}
