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

use nalgebra::{RealField, Vector3};

use crate::{Sdf, SdfT};

pub struct SdfScene<T: SdfT> {
    elements: Vec<Box<dyn Sdf<T>>>,
}

impl<T: SdfT> Sdf<T> for SdfScene<T> {
    fn run(&self, pos: &Vector3<T>) -> T {
        let mut smallest = T::max_value().unwrap();
        self.elements.iter().for_each(|elem| {
            let dist = elem.run(pos);
            smallest = RealField::min(smallest, dist);
        });

        smallest
    }
}

impl<T: SdfT> SdfScene<T> {
    pub fn from_vec(elems: Vec<Box<dyn Sdf<T>>>) -> Self {
        SdfScene { elements: elems }
    }
}
