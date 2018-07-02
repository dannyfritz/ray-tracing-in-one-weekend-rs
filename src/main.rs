#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_perf, clippy_correctness))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy_style, clippy_complexity, clippy_cargo))]
#![cfg_attr(feature = "cargo-clippy", allow(unknown_lints))]

extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod material;
mod pixel;
mod ray;
mod vec;

use camera::Camera;
use hitable::{Sphere, World};
use material::{Dialectric, Lambertian, Metal};
use pixel::{Pixel, Pixels};
use rand::{thread_rng, Rng};
use ray::Ray;
use std::rc::Rc;
use vec::Vec3;

const MAX_DEPTH: u32 = 50;
const NO_COLOR: Vec3 = Vec3(0.0, 0.0, 0.0);

fn color(r: &Ray, world: &World, depth: u32) -> Vec3 {
    if depth > MAX_DEPTH {
        return NO_COLOR;
    }
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(ref rec) => {
            if let Some((attenuation, scattered)) = rec.material.scatter(&r, rec) {
                return attenuation * color(&scattered, world, depth + 1);
            } else {
                return NO_COLOR;
            }
        }
        None => {
            let unit_direction = Vec3::unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let mut pixels = Pixels::new();
    let (w, h): (u32, u32) = (200, 100);
    let s = 100;
    let camera = Camera::new();
    let world = World {
        hitables: vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
            )),
            Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Rc::new(Dialectric::new(1.5)),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                -0.45,
                Rc::new(Dialectric::new(1.5)),
            )),
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
            )),
        ],
    };
    for y in 0..h {
        for x in 0..w {
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..s {
                let s_u = (x as f32 + thread_rng().gen_range(0.0, 1.0)) / w as f32;
                let s_v = (y as f32 + thread_rng().gen_range(0.0, 1.0)) / h as f32;
                let ray = camera.get_ray(s_u, s_v);
                pixel += color(&ray, &world, 0);
            }
            pixel /= s as f32;
            let pixel = Vec3::new(pixel.x().sqrt(), pixel.y().sqrt(), pixel.z().sqrt());
            pixels.push(Pixel::RGB8(pixel));
        }
    }
    image::save_buffer(
        "image.png",
        &pixels.create_buffer(),
        200,
        100,
        image::RGBA(8),
    ).unwrap()
}
