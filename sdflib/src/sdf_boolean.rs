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

use na::{ComplexField, Point3};
use nalgebra as na;

use crate::{Sdf, SdfT};

pub struct SdfSubtract<T: SdfT> {
    pub remove: Box<dyn Sdf<T>>,
    pub from: Box<dyn Sdf<T>>,
}

impl<T: SdfT> Sdf<T> for SdfSubtract<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let remove_dist = -self.remove.run(pos);
        let keep_dist = self.from.run(pos);

        if remove_dist > keep_dist {
            remove_dist
        } else {
            keep_dist
        }
    }
}

pub struct SdfUnionSmooth<T: SdfT> {
    pub a: Box<dyn Sdf<T>>,
    pub b: Box<dyn Sdf<T>>,
    pub smooth: T,
}

fn calc_union_smooth<T: SdfT>(a: &T, b: &T, k: &T) -> T {
    let h_part = *k - ComplexField::abs(*a - *b);
    let h = *na::partial_max(&h_part, &T::from_f32(0_f32).unwrap()).unwrap();

    let min = *na::partial_min(a, b).unwrap();
    min - h * h * T::from_f32(0.25_f32).unwrap() / *k
}

impl<T: SdfT> Sdf<T> for SdfUnionSmooth<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let a_dist = self.a.run(pos);
        let b_dist = self.b.run(pos);
        calc_union_smooth(&a_dist, &b_dist, &self.smooth)
    }
}

pub struct SdfSubtractSmooth<T: SdfT> {
    pub remove: Box<dyn Sdf<T>>,
    pub from: Box<dyn Sdf<T>>,
    pub smooth: T,
}

impl<T: SdfT> Sdf<T> for SdfSubtractSmooth<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let remove_dist = self.remove.run(pos);
        let keep_dist = self.from.run(pos);

        -calc_union_smooth(&remove_dist, &-keep_dist, &self.smooth)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn inside_box() {
//         let bx = SdfBox {
//             dims: Vector3::<f32>::new(1f32, 1f32, 1f32),
//         };
//         let result = bx.run(&Vector3::new(0f32, 0f32, 0.5f32));
//         assert_eq!(result < 0f32, true);
//     }

//     #[test]
//     fn outside_box() {
//         let bx = SdfBox {
//             dims: Vector3::<f32>::new(1f32, 1f32, 1f32),
//         };
//         let result = bx.run(&Vector3::new(0f32, 0f32, 1.5f32));
//         assert_eq!(result > 0f32, true);
//     }
// }
