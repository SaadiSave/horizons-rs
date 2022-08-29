#![warn(clippy::pedantic)]

pub mod api;

#[cfg(test)]
pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;

mod macros;

#[allow(clippy::wildcard_imports)]
pub(crate) use macros::*;
