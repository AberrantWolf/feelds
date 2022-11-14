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

use image::ImageBuffer;
use nalgebra::{RealField, Vector3};

use sdflib::{Sdf, SdfSphere};

fn march_rays<T>(origin: &Vector3<T>, direction: &Vector3<T>) -> image::Rgb<u8>
where
    T: RealField + Copy,
{
    let sph = SdfSphere::<T> {
        radius: T::from_f32(1.0).unwrap(),
    };

    let mut dist = sph.run(origin);
    let mut new_origin = origin + direction * dist;

    let contact = T::from_f32(0.0001).unwrap();
    let nothing = T::from_f32(1000.0).unwrap();

    loop {
        if dist < contact {
            return image::Rgb([255, 255, 255]);
        }

        if dist > nothing {
            return image::Rgb([0, 0, 0]);
        }

        new_origin = new_origin + direction * dist;
        dist = sph.run(&new_origin);
    }
}

fn main() {
    let width = 1280u32;
    let height = 720u32;
    let aspect = width as f32 / height as f32;
    let zoom = 1f32;

    let cam_pos = Vector3::new(0f32, 0f32, -2f32);
    let cam_fwd = Vector3::new(0f32, 0f32, 1f32);
    let cam_right = Vector3::new(0f32, 1f32, 0f32).cross(&cam_fwd);
    let cam_up = cam_fwd.cross(&cam_right);

    let ray_ctr = cam_pos + cam_fwd * zoom;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let x_ratio = x as f32 / width as f32;
        let y_ratio = y as f32 / height as f32;

        let u = 2f32 * x_ratio - 1f32;
        let v = 2f32 * y_ratio - 1f32;

        let px_point = ray_ctr + u * cam_right * aspect + v * cam_up;
        let direction = px_point - cam_pos;

        // let ucolor = (255.0 * px_point.x.abs()) as u8;
        // let vcolor = (255.0 * px_point.y.abs()) as u8;
        // let col = image::Rgb([ucolor, vcolor, 0u8]);
        let col = march_rays(&cam_pos, &direction);

        col
    });

    img.save("test.png").unwrap()
}
