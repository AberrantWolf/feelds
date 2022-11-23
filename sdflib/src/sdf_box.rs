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

use glam::Vec3A;

use crate::{Sdf, SdfCalc};

pub struct SdfBox {
    pub dims: Vec3A,
}

impl Sdf for SdfBox {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let q = pos.abs() - self.dims;
        SdfCalc {
            dist: q.max(Vec3A::ZERO).length() + q.max_element().min(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_box() {
        let bx = SdfBox {
            dims: Vec3A::new(1f32, 1f32, 1f32),
        };
        let result = bx.run(&Vec3A::new(0f32, 0f32, 0.5f32));
        assert_eq!(result.dist < 0f32, true);
    }

    #[test]
    fn outside_box() {
        let bx = SdfBox {
            dims: Vec3A::new(1f32, 1f32, 1f32),
        };
        let result = bx.run(&Vec3A::new(0f32, 0f32, 1.5f32));
        assert_eq!(result.dist > 0f32, true);
    }
}
