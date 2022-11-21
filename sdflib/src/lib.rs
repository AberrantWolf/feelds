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

use na::{Point3, RealField};
use nalgebra as na;

// An implementation of SDF functions.
// Based on https://iquilezles.org/articles/distfunctions/

mod sdf_alteration;
mod sdf_boolean;
mod sdf_box;
mod sdf_plane;
mod sdf_scene;
mod sdf_sphere;
mod sdf_transform;

pub use sdf_alteration::*;
pub use sdf_boolean::*;
pub use sdf_box::*;
pub use sdf_plane::*;
pub use sdf_scene::*;
pub use sdf_sphere::*;
pub use sdf_transform::*;

pub trait SdfT: RealField + Copy + From<f32> {}

impl SdfT for f32 {}
impl SdfT for f64 {}

pub trait Sdf<T: SdfT> {
    fn run(&self, pos: &Point3<T>) -> T;
}
