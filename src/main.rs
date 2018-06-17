extern crate image;

mod vec3;

use vec3::Vec3;

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

fn main() {
    let mut pixels = Pixels::new();
    let (w, h): (u32, u32) = (200, 100);
    for y in 0..h {
        for x in 0..w {
            let color = Vec3::new(x as f32 / w as f32, y as f32 / h as f32, 0.2);
            pixels.push(Pixel::RGB8(color));
        }
    }
    image::save_buffer("image.png", &pixels.to_buffer(), 200, 100, image::RGBA(8)).unwrap()
}
