use na::{RealField, Vector3};
use nalgebra as na;

use crate::Sdf;

pub struct Sphere<T> {
    radius: T,
}

impl<T: RealField + Copy> Sdf<T> for Sphere<T> {
    fn run(&self, pos: &Vector3<T>) -> T {
        pos.magnitude() - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_sphere() {
        let sph = Sphere { radius: 1f32 };
        let result = sph.run(&Vector3::new(0f32, 0f32, 0.5f32));
        assert_eq!(result < 0f32, true);
    }

    #[test]
    fn outside_sphere() {
        let sph = Sphere { radius: 1f32 };
        let result = sph.run(&Vector3::new(0f32, 0f32, 1.5f32));
        assert_eq!(result > 0f32, true);
    }
}
