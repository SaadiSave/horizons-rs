#[cfg(feature = "uom")]
unit! {
    system: uom::si;
    quantity: uom::si::velocity;

    @au_per_day: 1_731_456.9; "AU/d", "AU per day", "astronomical units per day";
    @kilometre_per_day: 86.4; "km/d", "km per day", "kilometres per day";
}

/// Coefficient in m/s
pub const AU_PER_DAY: f64 = 1_731_456.9;

/// Coefficient in m/s
pub const KILOMETRE_PER_SECOND: f64 = 1000.;

/// Coefficient in m/s
pub const KILOMETRE_PER_DAY: f64 = 86.4;
