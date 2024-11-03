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

pub struct SdfPlane {
    pub normal: Vec3A,
    pub offset: f32,
}

impl Sdf for SdfPlane {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        SdfCalc {
            dist: pos.dot(self.normal) - self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn above_plane() {
        let normal = Vec3A::new(0_f32, 1_f32, 0_f32).normalize();
        let ob = SdfPlane {
            normal,
            offset: 0f32,
        };
        let result = ob.run(&Vec3A::new(0f32, 0.1f32, 0f32));
        assert_eq!(result.dist > 0f32, true);
    }

    #[test]
    fn below_plane() {
        let normal = Vec3A::new(0_f32, 1_f32, 0_f32).normalize();
        let ob = SdfPlane {
            normal,
            offset: 0f32,
        };
        let result = ob.run(&Vec3A::new(0f32, -0.1f32, 0f32));
        assert_eq!(result.dist < 0f32, true);
    }

    #[test]
    fn below_plane_elevated() {
        let normal = Vec3A::new(0_f32, 1_f32, 0_f32).normalize();
        let ob = SdfPlane {
            normal,
            offset: 1f32,
        };
        let result = ob.run(&Vec3A::new(0_f32, 0.5_f32, 0_f32));
        println!("result: {}", result.dist);
        assert_eq!(result.dist < 0f32, true);
    }
}
