// Copyright 2022-2024 Scott Harper
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

use glam::Vec3A;

use crate::{Sdf, SdfCalc};

// ===== Sharp booleans =====
pub struct SdfUnion {
    pub a: Box<dyn Sdf + Sync>,
    pub b: Box<dyn Sdf + Sync>,
}

pub struct SdfSubtraction {
    pub remove: Box<dyn Sdf + Sync>,
    pub from: Box<dyn Sdf + Sync>,
}

pub struct SdfIntersection {
    pub a: Box<dyn Sdf + Sync>,
    pub b: Box<dyn Sdf + Sync>,
}

impl Sdf for SdfUnion {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let a_calc = self.a.run(pos);
        let b_calc = self.b.run(pos);

        a_calc.min(b_calc)
    }
}

impl Sdf for SdfSubtraction {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let remove_calc = self.remove.run(pos);
        let keep_calc = self.from.run(pos);

        SdfCalc {
            dist: (-remove_calc.dist).max(keep_calc.dist),
        }
    }
}

impl Sdf for SdfIntersection {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let a_dist = self.a.run(pos);
        let b_dist = self.b.run(pos);
        a_dist.max(b_dist)
    }
}

// ===== Smooth booleans =====
pub struct SdfUnionSmooth {
    pub a: Box<dyn Sdf + Sync>,
    pub b: Box<dyn Sdf + Sync>,
    pub smooth: f32,
}

pub struct SdfSubtractionSmooth {
    pub remove: Box<dyn Sdf + Sync>,
    pub from: Box<dyn Sdf + Sync>,
    pub smooth: f32,
}

pub struct SdfIntersectionSmooth {
    pub a: Box<dyn Sdf + Sync>,
    pub b: Box<dyn Sdf + Sync>,
    pub smooth: f32,
}

fn calc_union_smooth(a: f32, b: f32, k: f32) -> f32 {
    let h_part = k - (a - b).abs();
    let h = h_part.max(0.0);

    let min = a.min(b);
    min - h * h * 0.25 / k
}

impl Sdf for SdfUnionSmooth {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let a_calc = self.a.run(pos);
        let b_calc = self.b.run(pos);
        SdfCalc {
            dist: calc_union_smooth(a_calc.dist, b_calc.dist, self.smooth),
        }
    }
}

impl Sdf for SdfSubtractionSmooth {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let remove_calc = self.remove.run(pos);
        let keep_calc = self.from.run(pos);

        SdfCalc {
            dist: -calc_union_smooth(remove_calc.dist, -keep_calc.dist, self.smooth),
        }
    }
}

impl Sdf for SdfIntersectionSmooth {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let a_calc = self.a.run(pos);
        let b_calc = self.a.run(pos);

        SdfCalc {
            dist: -calc_union_smooth(-a_calc.dist, -b_calc.dist, self.smooth),
        }
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
