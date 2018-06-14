extern crate image;

enum Pixel {
    RGB8(u8, u8, u8),
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
                Pixel::RGB8(r, g, b) => {
                    buffer.push(r);
                    buffer.push(g);
                    buffer.push(b);
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
            pixels.push(Pixel::RGB8(
                ((x as f32 / w as f32) * u8::max_value() as f32) as u8,
                (((h - y) as f32 / h as f32) * u8::max_value() as f32) as u8,
                0,
            ));
        }
    }
    image::save_buffer("image.png", &pixels.to_buffer(), 200, 100, image::RGBA(8)).unwrap()
}
