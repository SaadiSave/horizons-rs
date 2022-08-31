#![warn(clippy::pedantic)]

pub mod request;
pub mod response;

pub(crate) mod units;

#[cfg(test)]
pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;

mod macros;

#[allow(clippy::wildcard_imports)]
pub(crate) use macros::*;

#[cfg(feature = "uom")]
#[macro_use]
extern crate uom;
