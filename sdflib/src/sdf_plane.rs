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

use nalgebra::{Point3, UnitVector3};

use crate::{Sdf, SdfT};

pub struct SdfPlane<T> {
    pub normal: UnitVector3<T>,
    pub offset: T,
}

impl<T: SdfT> Sdf<T> for SdfPlane<T> {
    fn run(&self, pos: &Point3<T>) -> T {
        pos.coords.dot(&self.normal) - self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn above_plane() {
        let normal = UnitVector3::new_normalize(Vector3::new(0_f32, 1_f32, 0_f32));
        let ob = SdfPlane {
            normal,
            offset: 0f32,
        };
        let result = ob.run(&Point3::new(0f32, 0.1f32, 0f32));
        assert_eq!(result > 0f32, true);
    }

    #[test]
    fn below_plane() {
        let normal = UnitVector3::new_normalize(Vector3::new(0_f32, 1_f32, 0_f32));
        let ob = SdfPlane {
            normal,
            offset: 0f32,
        };
        let result = ob.run(&Point3::new(0f32, -0.1f32, 0f32));
        assert_eq!(result < 0f32, true);
    }

    #[test]
    fn below_plane_elevated() {
        let normal = UnitVector3::new_normalize(Vector3::new(0_f32, 1_f32, 0_f32));
        let ob = SdfPlane {
            normal,
            offset: 1f32,
        };
        let result = ob.run(&Point3::new(0_f32, 0.5_f32, 0_f32));
        println!("result: {}", result);
        assert_eq!(result < 0f32, true);
    }
}
