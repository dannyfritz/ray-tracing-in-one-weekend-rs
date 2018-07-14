#![cfg_attr(
    feature = "cargo-clippy",
    deny(clippy, clippy_perf, clippy_correctness)
)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy_style, clippy_complexity)
)]
#![cfg_attr(feature = "cargo-clippy", allow(unknown_lints))]

extern crate image;
extern crate nalgebra;
extern crate ncollide3d;
extern crate rand;
extern crate rayon;

mod camera;
mod material;
mod pixel;
mod scene;
mod utility;

use material::color;
use ncollide3d::math::Vector;
use pixel::{Pixel, Pixels};
use rayon::prelude::*;
#[allow(unused_imports)]
use scene::{cornell_box_scene, random_scene, structured_art_scene};
use utility::random::rand;

const MAX_DEPTH: u32 = 30;
const NUM_SAMPLES: u32 = 100;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

fn main() {
    let mut pixels = Pixels::new();
    let (world, camera) = random_scene();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut pixel: Vector<f32> = (0u32..NUM_SAMPLES)
                .into_par_iter()
                .map(|_| {
                    let s_u = (x as f32 + rand()) / WIDTH as f32;
                    let s_v = (y as f32 + rand()) / HEIGHT as f32;
                    let ray = camera.get_ray(s_u, s_v);
                    color(&ray, &world, 0)
                })
                .reduce(|| Vector::new(0.0, 0.0, 0.0), |pixel, c| pixel + c);
            pixel /= NUM_SAMPLES as f32;
            let pixel = Vector::new(pixel.x.sqrt(), pixel.y.sqrt(), pixel.z.sqrt());
            pixels.push(Pixel::RGB8(pixel));
        }
        if y % 10 == 0 {
            println!("row {} of {}", y, HEIGHT);
        }
    }
    image::save_buffer(
        "image.png",
        &pixels.create_buffer(),
        WIDTH,
        HEIGHT,
        image::RGBA(8),
    ).unwrap();
}
