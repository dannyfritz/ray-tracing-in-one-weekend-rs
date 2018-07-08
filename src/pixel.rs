use ncollide3d::math::Vector;

pub enum Pixel {
    RGB8(Vector<f32>),
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
    pub fn create_buffer(self) -> Box<[u8]> {
        let mut buffer = Vec::new();
        for pixel in self.0 {
            match pixel {
                Pixel::RGB8(v) => {
                    buffer.push((v.x * f32::from(u8::max_value())) as u8);
                    buffer.push((v.y * f32::from(u8::max_value())) as u8);
                    buffer.push((v.z * f32::from(u8::max_value())) as u8);
                    buffer.push(u8::max_value());
                }
            };
        }
        buffer.into_boxed_slice()
    }
}
