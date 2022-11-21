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

// ===== Sharp booleans =====
pub struct SdfUnion<T: SdfT> {
    pub a: Box<dyn Sdf<T>>,
    pub b: Box<dyn Sdf<T>>,
}

pub struct SdfSubtraction<T: SdfT> {
    pub remove: Box<dyn Sdf<T>>,
    pub from: Box<dyn Sdf<T>>,
}

pub struct SdfIntersection<T: SdfT> {
    pub a: Box<dyn Sdf<T>>,
    pub b: Box<dyn Sdf<T>>,
}

impl<T: SdfT> Sdf<T> for SdfUnion<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let a_dist = self.a.run(pos);
        let b_dist = self.b.run(pos);
        *na::partial_min(&a_dist, &b_dist).unwrap()
    }
}

impl<T: SdfT> Sdf<T> for SdfSubtraction<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let remove_dist = self.remove.run(pos);
        let keep_dist = self.from.run(pos);
        *na::partial_max(&-remove_dist, &keep_dist).unwrap()
    }
}

impl<T: SdfT> Sdf<T> for SdfIntersection<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let a_dist = self.a.run(pos);
        let b_dist = self.b.run(pos);
        *na::partial_max(&a_dist, &b_dist).unwrap()
    }
}

// ===== Smooth booleans =====
pub struct SdfUnionSmooth<T: SdfT> {
    pub a: Box<dyn Sdf<T>>,
    pub b: Box<dyn Sdf<T>>,
    pub smooth: T,
}

pub struct SdfSubtractionSmooth<T: SdfT> {
    pub remove: Box<dyn Sdf<T>>,
    pub from: Box<dyn Sdf<T>>,
    pub smooth: T,
}

pub struct SdfIntersectionSmooth<T: SdfT> {
    pub a: Box<dyn Sdf<T>>,
    pub b: Box<dyn Sdf<T>>,
    pub smooth: T,
}

fn calc_union_smooth<T: SdfT>(a: &T, b: &T, k: &T) -> T {
    let h_part = *k - ComplexField::abs(*a - *b);
    let h = *na::partial_max(&h_part, &0.0.into()).unwrap();

    let min = *na::partial_min(a, b).unwrap();
    min - h * h * 0.25.into() / *k
}

impl<T: SdfT> Sdf<T> for SdfUnionSmooth<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let a_dist = self.a.run(pos);
        let b_dist = self.b.run(pos);
        calc_union_smooth(&a_dist, &b_dist, &self.smooth)
    }
}

impl<T: SdfT> Sdf<T> for SdfSubtractionSmooth<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let remove_dist = self.remove.run(pos);
        let keep_dist = self.from.run(pos);

        -calc_union_smooth(&remove_dist, &-keep_dist, &self.smooth)
    }
}

impl<T: SdfT> Sdf<T> for SdfIntersectionSmooth<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let a_dist = self.a.run(pos);
        let b_dist = self.a.run(pos);

        -calc_union_smooth(&-a_dist, &-b_dist, &self.smooth)
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
