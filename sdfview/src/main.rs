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
use nalgebra::Vector3;
use num::cast::AsPrimitive;

use sdflib::{Sdf, SdfBox, SdfScene, SdfSphere, SdfSubtract, SdfT};

fn calc_normal<T>(scene: &Box<dyn Sdf<T>>, point: &Vector3<T>) -> image::Rgb<u8>
where
    T: SdfT + AsPrimitive<u8> + From<f32>,
{
    let small_step = Vector3::<T>::new(0.001f32.into(), 0f32.into(), 0f32.into());

    let grad_x = scene.run(&(point + small_step.xyy())) - scene.run(&(point - small_step.xyy()));
    let grad_y = scene.run(&(point + small_step.yxy())) - scene.run(&(point - small_step.yxy()));
    let grad_z = scene.run(&(point + small_step.yyx())) - scene.run(&(point - small_step.yyx()));

    let normal: Vector3<T> = Vector3::new(grad_x, grad_y, grad_z)
        .normalize()
        .scale(0.5f32.into())
        .add_scalar(0.5f32.into());
    let r = T::from(255_f32) * normal.x;
    let g = T::from(255_f32) * normal.y;
    let b = T::from(255_f32) * normal.z;

    image::Rgb([r.as_(), g.as_(), b.as_()])
}

fn march_rays<T>(
    scene: &Box<dyn Sdf<T>>,
    origin: &Vector3<T>,
    direction: &Vector3<T>,
) -> image::Rgb<u8>
where
    T: SdfT + AsPrimitive<u8> + From<f32>,
{
    let mut dist;
    let mut new_origin = *origin;

    let contact = 0.001_f32.into();
    let nothing = 1000.0_f32.into();

    loop {
        dist = scene.run(&new_origin);
        if dist < contact {
            return calc_normal::<T>(scene, &new_origin);
        }

        if dist > nothing {
            return image::Rgb([63, 0, 63]);
        }
        new_origin = new_origin + direction.scale(dist);
    }
}

fn main() {
    // == final image settings ==
    let width = 1280_u32;
    let height = 720_u32;
    let aspect = width as f32 / height as f32;

    // == setting up camera stuff ==
    let zoom = 1_f32;

    let cam_pos = Vector3::new(2_f32, 1_f32, -2_f32);
    let cam_target = Vector3::new(0_f32, 0_f32, 0_f32);
    let cam_fwd = (cam_target - cam_pos).normalize();
    let cam_right = Vector3::new(0_f32, 1_f32, 0_f32).cross(&cam_fwd);
    let cam_up = cam_fwd.cross(&cam_right);

    let ray_ctr = cam_pos + cam_fwd * zoom;

    // == making the scene ==
    let the_box = Box::new(SdfBox {
        dims: Vector3::new(0.75_f32, 0.75_f32, 0.75_f32),
    });
    let the_sphere = Box::new(SdfSphere { radius: 1_f32 });

    let bool_thing: Box<dyn Sdf<_>> = Box::new(SdfSubtract {
        remove: the_sphere,
        from: the_box,
    });

    let scene: Box<dyn Sdf<_>> = Box::new(SdfScene::from_vec(vec![bool_thing]));

    // == rendering the image
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let x_ratio = x as f32 / width as f32;
        let y_ratio = y as f32 / height as f32;

        let u = 2f32 * x_ratio - 1f32;
        let v = 2f32 * y_ratio - 1f32;

        let px_point = ray_ctr + u * cam_right * aspect - v * cam_up;
        let direction = (px_point - cam_pos).normalize();

        march_rays::<f32>(&scene, &cam_pos, &direction)
    });

    img.save("test.png").unwrap()
}
