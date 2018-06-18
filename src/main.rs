extern crate image;

mod ray;
mod vec;

use ray::Ray;
use vec::Vec3;

enum Pixel {
    RGB8(Vec3),
    // RGBA8(u8, u8, u8, u8),
}

struct Pixels(Vec<Pixel>);

impl Pixels {
    fn new() -> Pixels {
        Pixels(Vec::new())
    }
    fn push(&mut self, pixel: Pixel) {
        self.0.push(pixel);
    }
    fn to_buffer(self) -> Box<[u8]> {
        let mut buffer = Vec::new();
        for pixel in self.0 {
            match pixel {
                Pixel::RGB8(v) => {
                    buffer.push((v.r() * u8::max_value() as f32) as u8);
                    buffer.push((v.g() * u8::max_value() as f32) as u8);
                    buffer.push((v.b() * u8::max_value() as f32) as u8);
                    buffer.push(u8::max_value());
                }
            };
        }
        buffer.into_boxed_slice()
    }
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut pixels = Pixels::new();
    let (w, h): (u32, u32) = (200, 100);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / w as f32;
            let v = y as f32 / h as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + (1.0 - v) * vertical);
            pixels.push(Pixel::RGB8(color(&r)));
        }
    }
    image::save_buffer("image.png", &pixels.to_buffer(), 200, 100, image::RGBA(8)).unwrap()
}
