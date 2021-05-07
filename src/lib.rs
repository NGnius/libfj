//! An unofficial collection of APIs used in Robocraft and Cardlife.
//!
//! This crate is WIP, but the available APIs are tested and very usable.
#![warn(missing_docs)]

pub mod cardlife;
pub mod robocraft;
#[cfg(feature = "simple")]
pub mod robocraft_simple;
#[cfg(feature = "simple")]
pub mod cardlife_simple;
