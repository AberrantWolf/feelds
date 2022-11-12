use na::{RealField, Vector3};
use nalgebra as na;

use crate::Sdf;

pub struct SdfBox<T> {
    dims: Vector3<T>,
}

impl<T: RealField + Copy> Sdf<T> for SdfBox<T> {
    fn run(&self, pos: &Vector3<T>) -> T {
        let q = pos.abs() - self.dims;
        let zero = T::from_f64(0.0).unwrap();
        q.map(|t| t.max(zero)).magnitude() + q.max().min(zero)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_box() {
        let bx = SdfBox {
            dims: Vector3::<f32>::new(1f32, 1f32, 1f32),
        };
        let result = bx.run(&Vector3::new(0f32, 0f32, 0.5f32));
        assert_eq!(result < 0f32, true);
    }

    #[test]
    fn outside_box() {
        let bx = SdfBox {
            dims: Vector3::<f32>::new(1f32, 1f32, 1f32),
        };
        let result = bx.run(&Vector3::new(0f32, 0f32, 1.5f32));
        assert_eq!(result > 0f32, true);
    }
}
