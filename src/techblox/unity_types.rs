use crate::techblox::{Parsable};
use libfj_parsable_macro_derive::*;

/// Unity-like floating-point vector for 3-dimensional space.
#[derive(Clone, Copy, Parsable)]
pub struct UnityFloat3 {
    /// x coordinate
    pub x: f32,
    /// y coordinate
    pub y: f32,
    /// z coordinate
    pub z: f32,
}

/// Unity-like floating-point vector for 4-dimensional space.
#[derive(Clone, Copy, Parsable)]
pub struct UnityFloat4 {
    /// x coordinate
    pub x: f32,
    /// y coordinate
    pub y: f32,
    /// z coordinate
    pub z: f32,
    /// w coordinate
    pub w: f32,
}

/// Unity-like floating-point vector matrix for 4-dimensional space.
#[derive(Clone, Copy, Parsable)]
pub struct UnityFloat4x4 {
    /// c0 row(?)
    pub c0: UnityFloat4,
    /// c1 row(?)
    pub c1: UnityFloat4,
    /// c2 row(?)
    pub c2: UnityFloat4,
    /// c3 row(?)
    pub c3: UnityFloat4,
}

/// Unity-like floating-point quaternion for rotation in 3-dimensional space.
#[derive(Clone, Copy, Parsable)]
pub struct UnityQuaternion {
    /// Rotational orientation
    pub value: UnityFloat4,
}
