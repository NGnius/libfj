//! An unofficial collection of APIs used in Robocraft and Cardlife.
//!
//! This crate is WIP, but the available APIs are tested and very usable.
#[cfg(feature = "cardlife")]
pub mod cardlife;
#[cfg(all(feature = "simple", feature = "cardlife"))]
pub mod cardlife_simple;
#[cfg(feature = "robocraft")]
pub mod robocraft;
#[cfg(all(feature = "simple", feature = "robocraft"))]
pub mod robocraft_simple;
#[cfg(feature = "techblox")]
pub mod techblox;
#[cfg(feature = "convert")]
pub mod convert;
#[cfg(feature = "robocraft")]
pub mod robocraft2;
