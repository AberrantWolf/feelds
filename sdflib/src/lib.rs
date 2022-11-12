use na::{RealField, Vector3};
use nalgebra as na;

// An implementation of SDF functions.
// Based on https://iquilezles.org/articles/distfunctions/

pub mod sdf_box;
pub mod sdf_sphere;

pub trait Sdf<T: RealField> {
    fn run(&self, pos: &Vector3<T>) -> T;
}
