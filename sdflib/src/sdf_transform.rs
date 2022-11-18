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

use nalgebra::IsometryMatrix3;
use nalgebra::Point3;

use crate::{Sdf, SdfT};

pub struct SdfTransform<T> {
    invx: IsometryMatrix3<T>,
    elem: Box<dyn Sdf<T>>,
}

impl<T: SdfT> Sdf<T> for SdfTransform<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        let xfpos = self.invx.transform_point(pos);
        self.elem.run(&xfpos)
    }
}

impl<T: SdfT> SdfTransform<T> {
    pub fn new(xform: IsometryMatrix3<T>, elem: Box<dyn Sdf<T>>) -> Self {
        let invx = xform.inverse();
        SdfTransform { invx, elem }
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
