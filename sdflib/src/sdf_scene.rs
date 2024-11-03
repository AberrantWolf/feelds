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

pub struct SdfScene {
    elements: Vec<Box<dyn Sdf + Sync>>,
}

impl Sdf for SdfScene {
    fn run(&self, pos: &Vec3A) -> SdfCalc {
        let mut smallest = f32::MAX;
        self.elements.iter().for_each(|elem| {
            let calc = elem.run(pos);
            smallest = smallest.min(calc.dist);
        });

        SdfCalc { dist: smallest }
    }
}

impl SdfScene {
    pub fn from_vec(elems: Vec<Box<dyn Sdf + Sync>>) -> Self {
        SdfScene { elements: elems }
    }
}
