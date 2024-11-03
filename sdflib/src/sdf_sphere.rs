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

pub struct SdfSphere {
    pub radius: f32,
}

impl Sdf for SdfSphere {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        SdfCalc {
            dist: pos.length() - self.radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_sphere() {
        let sph = SdfSphere { radius: 1f32 };
        let result = sph.run(&Vec3A::new(0f32, 0f32, 0.5f32));
        assert_eq!(result.dist < 0f32, true);
    }

    #[test]
    fn outside_sphere() {
        let sph = SdfSphere { radius: 1f32 };
        let result = sph.run(&Vec3A::new(0f32, 0f32, 1.5f32));
        assert_eq!(result.dist > 0f32, true);
    }
}
