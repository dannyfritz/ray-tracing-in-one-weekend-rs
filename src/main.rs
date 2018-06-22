extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod pixel;
mod ray;
mod vec;

use camera::Camera;
use hitable::{Hitable, Sphere, World};
use pixel::{Pixel, Pixels};
use rand::{thread_rng, Rng};
use ray::Ray;
use vec::Vec3;

fn color(r: Ray, world: &World) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(ref rec) => {
            let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            0.5 * color(Ray::new(rec.p, target - rec.p), world)
        }
        None => {
            let unit_direction = Vec3::unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
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
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };
    for y in 0..h {
        for x in 0..w {
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..s {
                let u = (x as f32 + thread_rng().gen_range(0.0, 1.0)) / w as f32;
                let v = (y as f32 + thread_rng().gen_range(0.0, 1.0)) / h as f32;
                let r = camera.get_ray(u, v);
                pixel += color(r, &world);
            }
            pixel /= s as f32;
            pixels.push(Pixel::RGB8(pixel));
        }
    }
    image::save_buffer("image.png", &pixels.to_buffer(), 200, 100, image::RGBA(8)).unwrap()
}
