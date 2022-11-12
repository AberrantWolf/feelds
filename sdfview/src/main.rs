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

use image::{ImageBuffer, RgbImage};

fn main() {
    let width = 1280u32;
    let height = 720u32;

    // TODO: Create a camera origin

    // TODO: Create a draw plane to shoot rays through

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        image::Rgb([((255 * x) / width) as u8, ((255 * y) / height) as u8, 0u8])
    });

    // TODO: Run some ray marching to generate hits (or misses, I guess)

    img.save("test.png").unwrap()
}
