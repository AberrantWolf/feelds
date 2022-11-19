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
use nalgebra::{IsometryMatrix3, Point3, Rotation3, Translation3, UnitVector3, Vector3};
use num::cast::AsPrimitive;

use sdflib::{
    Sdf, SdfBox, SdfPlane, SdfScene, SdfSmooth, SdfSphere, SdfSubtraction, SdfSubtractionSmooth,
    SdfT, SdfTransform, SdfUnionSmooth,
};

fn calc_normal<T>(scene: &Box<dyn Sdf<T>>, point: &Point3<T>) -> image::Rgb<u8>
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
    origin: &Point3<T>,
    direction: &Vector3<T>,
) -> image::Rgb<u8>
where
    T: SdfT + AsPrimitive<u8> + From<f32>,
{
    let mut dist;
    let mut new_origin = *origin;

    let contact = 0.001_f32.into();
    let nothing = 1000.0_f32.into();

    let max_same = 3u32;

    let mut last_dist = nothing;
    let mut same_count = 0u32;
    loop {
        dist = scene.run(&new_origin);
        if dist == last_dist {
            same_count += 1;
            if same_count >= max_same {
                break;
            }
        } else {
            last_dist = dist;
            same_count = 0;
        }

        if dist < contact {
            return calc_normal::<T>(scene, &new_origin);
        }

        if dist > nothing {
            break;
        }
        new_origin = new_origin + direction.scale(dist);
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

    let cam_pos = Point3::new(2_f32, 1_f32, -2_f32);
    let cam_target = Point3::new(0_f32, 0_f32, 0_f32);
    let cam_fwd = (cam_target - cam_pos).normalize();
    let cam_right = Vector3::new(0_f32, 1_f32, 0_f32).cross(&cam_fwd);
    let cam_up = cam_fwd.cross(&cam_right);

    let ray_ctr = cam_pos + cam_fwd * zoom;

    // == making the scene ==
    let the_box = Box::new(SdfSmooth {
        elem: Box::new(SdfBox {
            dims: Vector3::new(0.75_f32, 0.75_f32, 0.75_f32),
        }),
        smooth: 0.5_f32,
    });
    let the_sphere = Box::new(SdfTransform::new(
        IsometryMatrix3::from_parts(
            Translation3::new(0_f32, 1.25_f32, -0.5_f32),
            Rotation3::default(),
        ),
        Box::new(SdfSphere { radius: 0.7_f32 }),
    ));
    let bool_thing: Box<dyn Sdf<_>> = Box::new(SdfUnionSmooth {
        a: the_sphere,
        b: the_box,
        smooth: 0.2_f32,
    });

    let ground: Box<dyn Sdf<_>> = Box::new(SdfPlane {
        normal: UnitVector3::new_normalize(Vector3::new(0_f32, 1_f32, 0_f32)),
        offset: -1_f32,
    });

    let scene: Box<dyn Sdf<_>> = Box::new(SdfScene::from_vec(vec![bool_thing, ground]));

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
