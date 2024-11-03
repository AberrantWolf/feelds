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

use glam::Affine3A;
use image::{ImageBuffer, Rgb};
use num::cast::AsPrimitive;
use rayon::prelude::*;

use glam::{Quat, Vec3A, Vec3Swizzles};

use sdflib::{Sdf, SdfBox, SdfPlane, SdfScene, SdfSmooth, SdfSphere, SdfTransform, SdfUnionSmooth};

fn calc_normal(scene: &Box<dyn Sdf + Sync>, point: Vec3A) -> image::Rgb<u8> {
    let small_step = Vec3A::new(0.001f32.into(), 0f32.into(), 0f32.into());

    let grad_x =
        scene.run(&(point + small_step.xyy())).dist - scene.run(&(point - small_step.xyy())).dist;
    let grad_y =
        scene.run(&(point + small_step.yxy())).dist - scene.run(&(point - small_step.yxy())).dist;
    let grad_z =
        scene.run(&(point + small_step.yyx())).dist - scene.run(&(point - small_step.yyx())).dist;

    let normal: Vec3A = Vec3A::new(grad_x, grad_y, grad_z).normalize() * (0.5f32) + 0.5f32;
    let r = 255_f32 * normal.x;
    let g = 255_f32 * normal.y;
    let b = 255_f32 * normal.z;

    image::Rgb([r.as_(), g.as_(), b.as_()])
}

fn march_rays(scene: &Box<dyn Sdf + Sync>, origin: Vec3A, direction: Vec3A) -> image::Rgb<u8> {
    let mut calc;
    let mut new_origin = origin;

    let contact = 0.001_f32;
    let nothing = 1000.0_f32;

    let max_same = 3u32;

    let mut last_dist = nothing;
    let mut same_count = 0u32;
    loop {
        calc = scene.run(&new_origin);
        if calc.dist == last_dist {
            same_count += 1;
            if same_count >= max_same {
                break;
            }
        } else {
            last_dist = calc.dist;
            same_count = 0;
        }

        if calc.dist < contact {
            return calc_normal(scene, new_origin);
        }

        if calc.dist > nothing {
            break;
        }
        new_origin = new_origin + direction * calc.dist;
    }
    image::Rgb([63, 0, 63])
}

fn main() {
    // == final image settings ==
    let width = 1280_u32;
    let height = 720_u32;
    let aspect = width as f32 / height as f32;

    // == setting up camera stuff ==
    let zoom = 1_f32;

    let cam_pos = Vec3A::new(2_f32, 1_f32, -2_f32);
    let cam_target = Vec3A::new(0_f32, 0_f32, 0_f32);
    let cam_fwd = (cam_target - cam_pos).normalize();
    let cam_right = Vec3A::new(0_f32, 1_f32, 0_f32).cross(cam_fwd);
    let cam_up = cam_fwd.cross(cam_right);

    let ray_ctr = cam_pos + cam_fwd * zoom;

    // == making the scene ==
    let the_box = Box::new(SdfSmooth {
        elem: Box::new(SdfBox {
            dims: Vec3A::new(0.75_f32, 0.75_f32, 0.75_f32),
        }),
        smooth: 0.5_f32,
    });
    let the_sphere = Box::new(SdfTransform::new(
        Affine3A::from_rotation_translation(
            Quat::IDENTITY,
            Vec3A::new(0_f32, 1.25_f32, -0.5_f32).into(),
        ),
        Box::new(SdfSphere { radius: 0.7_f32 }),
    ));
    let bool_thing: Box<dyn Sdf + Sync> = Box::new(SdfUnionSmooth {
        a: the_sphere,
        b: the_box,
        smooth: 0.2_f32,
    });

    let ground: Box<dyn Sdf + Sync> = Box::new(SdfPlane {
        normal: Vec3A::new(0_f32, 1_f32, 0_f32).normalize(),
        offset: -1_f32,
    });

    let scene: Box<dyn Sdf + Sync> = Box::new(SdfScene::from_vec(vec![bool_thing, ground]));

    // == rendering the image
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let x_ratio = x as f32 / width as f32;
        let y_ratio = y as f32 / height as f32;

        let u = 2f32 * x_ratio - 1f32;
        let v = 2f32 * y_ratio - 1f32;

        let px_point = ray_ctr + u * cam_right * aspect - v * cam_up;
        let direction = (px_point - cam_pos).normalize();

        *pixel = march_rays(&scene, cam_pos, direction);
    });

    img.save("test.png").unwrap()
}
