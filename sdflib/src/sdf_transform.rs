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

use std::ops::Div;

use glam::Affine3A;
use glam::Vec3A;

use crate::{Sdf, SdfCalc};

pub struct SdfTransform {
    invx: Affine3A,
    elem: Box<dyn Sdf>,
}

impl Sdf for SdfTransform {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let xfpos = self.invx.transform_point3a(*pos);
        self.elem.run(&xfpos)
    }
}

impl SdfTransform {
    pub fn new(xform: Affine3A, elem: Box<dyn Sdf>) -> Self {
        let invx = xform.inverse();
        SdfTransform { invx, elem }
    }
}

pub struct SdfScale {
    elem: Box<dyn Sdf>,
    scale: f32,
}

impl Sdf for SdfScale {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let s_pos = pos.div(self.scale);
        SdfCalc {
            dist: self.elem.run(&s_pos).dist * self.scale,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn inside_sphere() {
//         let sph = SdfSphere { radius: 1f32 };
//         let result = sph.run(&Vector3::new(0f32, 0f32, 0.5f32));
//         assert_eq!(result < 0f32, true);
//     }

//     #[test]
//     fn outside_sphere() {
//         let sph = SdfSphere { radius: 1f32 };
//         let result = sph.run(&Vector3::new(0f32, 0f32, 1.5f32));
//         assert_eq!(result > 0f32, true);
//     }
// }
