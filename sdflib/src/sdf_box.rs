// Copyright 2022 Scott Harper
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use na::{Point3, Vector3};
use nalgebra as na;

use crate::{Sdf, SdfT};

pub struct SdfBox<T: SdfT> {
    pub dims: Vector3<T>,
}

impl<T: SdfT> Sdf<T> for SdfBox<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let q = pos.coords.abs() - self.dims;
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
        let result = bx.run(&Point3::new(0f32, 0f32, 0.5f32));
        assert_eq!(result < 0f32, true);
    }

    #[test]
    fn outside_box() {
        let bx = SdfBox {
            dims: Vector3::<f32>::new(1f32, 1f32, 1f32),
        };
        let result = bx.run(&Point3::new(0f32, 0f32, 1.5f32));
        assert_eq!(result > 0f32, true);
    }
}
