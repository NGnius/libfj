//! Conversion utility functions.
//!
//! This will contain ways to convert data from one game to another as well as into standard formats.

#[cfg(feature = "robocraft")]
mod robocraft_3d;
#[cfg(feature = "robocraft")]
pub use robocraft_3d::{cubes_to_model, cubes_to_model_with_lut, cube_rotation_to_quat};
