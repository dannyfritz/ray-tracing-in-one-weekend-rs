#![cfg_attr(
    feature = "cargo-clippy",
    deny(clippy, clippy_perf, clippy_correctness)
)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy_style, clippy_complexity, clippy_cargo)
)]
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
use hitable::Hitable;
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

fn random_scene() -> Vec<Box<dyn Hitable>> {
    let mut hitables: Vec<Box<dyn Hitable>> = vec![];
    hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = thread_rng().gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as f32 + 0.9 + thread_rng().gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 + thread_rng().gen_range(0.0, 1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitables.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Vec3::new(
                            thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                            thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                            thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    hitables.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + thread_rng().gen_range(0.0, 1.0)),
                                0.5 * (1.0 + thread_rng().gen_range(0.0, 1.0)),
                                0.5 * (1.0 + thread_rng().gen_range(0.0, 1.0)),
                            ),
                            0.5 + (1.0 + thread_rng().gen_range(0.0, 1.0)),
                        )),
                    )));
                } else {
                    hitables.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dialectric::new(1.5)),
                    )));
                }
            }
        }
    }
    hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dialectric::new(1.5)),
    )));
    hitables.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    hitables.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    hitables
}

fn main() {
    let mut pixels = Pixels::new();
    let (w, h): (u32, u32) = (800, 600);
    let total_pixels = w * h;
    let s = 200;
    let look_from = Vec3::new(10.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let distance_to_focus = (look_from - look_at).length();
    let aperture = 0.1;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &Vec3::new(0.0, 1.0, 0.0),
        25.0,
        w as f32 / h as f32,
        aperture,
        distance_to_focus,
    );
    let world = World {
        hitables: random_scene(),
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
        println!(
            "{:.2}% done. {} of {}",
            (y * w) as f32 / (total_pixels) as f32 * 100.0,
            y * w,
            total_pixels
        );
    }
    image::save_buffer("image.png", &pixels.create_buffer(), w, h, image::RGBA(8)).unwrap()
}
