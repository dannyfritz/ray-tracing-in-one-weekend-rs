use vec::Vec3;

pub enum Pixel {
    RGB8(Vec3),
    // RGBA8(u8, u8, u8, u8),
}

pub struct Pixels(Vec<Pixel>);

impl Pixels {
    pub fn new() -> Pixels {
        Pixels(Vec::new())
    }
    pub fn push(&mut self, pixel: Pixel) {
        self.0.push(pixel);
    }
    pub fn to_buffer(self) -> Box<[u8]> {
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
