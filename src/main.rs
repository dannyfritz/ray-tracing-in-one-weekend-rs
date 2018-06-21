extern crate image;

mod hitable;
mod pixel;
mod ray;
mod vec;

use hitable::{Hitable, Sphere, World};
use pixel::{Pixel, Pixels};
use ray::Ray;
use vec::Vec3;

fn color(r: &Ray, world: &World) -> Vec3 {
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(ref rec) => {
            0.5 * Vec3::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            )
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
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world = World {
        hitables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / w as f32;
            let v = y as f32 / h as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + (1.0 - v) * vertical,
            );
            pixels.push(Pixel::RGB8(color(&r, &world)));
        }
    }
    image::save_buffer("image.png", &pixels.to_buffer(), 200, 100, image::RGBA(8)).unwrap()
}
