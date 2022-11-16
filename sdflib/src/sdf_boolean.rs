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

use na::Vector3;
use nalgebra as na;

use crate::{Sdf, SdfT};

pub struct SdfSubtract<T: SdfT> {
    pub remove: Box<dyn Sdf<T>>,
    pub from: Box<dyn Sdf<T>>,
}

impl<T: SdfT> Sdf<T> for SdfSubtract<T> {
    fn run(&self, pos: &Vector3<T>) -> T {
        let remove_dist = -self.remove.run(pos);
        let keep_dist = self.from.run(pos);

        if remove_dist > keep_dist {
            remove_dist
        } else {
            keep_dist
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
