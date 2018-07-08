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
extern crate nalgebra;
extern crate ncollide3d;
extern crate rand;

#[cfg(feature = "profile")]
extern crate flame;

mod camera;
mod material;
mod pixel;
mod scene;
mod utility;

use material::color;
use ncollide3d::math::Vector;
use pixel::{Pixel, Pixels};
#[allow(unused_imports)]
use scene::{random_scene, structured_art_scene};
use utility::random::rand;
use utility::profile;

const MAX_DEPTH: u32 = 50;
const NUM_SAMPLES: u32 = 100;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

fn main() {
    profile::start("main");
    let mut pixels = Pixels::new();
    let (world, camera) = random_scene();
    profile::start("ray casting");
    for y in 0..HEIGHT {
        profile::start("ray casting row");
        for x in 0..WIDTH {
            profile::start("ray casting pixel");
            let mut pixel = Vector::new(0.0, 0.0, 0.0);
            for _s in 0..NUM_SAMPLES {
                let s_u = (x as f32 + rand()) / WIDTH as f32;
                let s_v = (y as f32 + rand()) / HEIGHT as f32;
                let ray = camera.get_ray(s_u, s_v);
                pixel += color(&ray, world.clone(), 0);
            }
            pixel /= NUM_SAMPLES as f32;
            profile::start("store pixel");
            let pixel = Vector::new(pixel.x.sqrt(), pixel.y.sqrt(), pixel.z.sqrt());
            pixels.push(Pixel::RGB8(pixel));
            profile::end("store pixel");
            profile::end("ray casting pixel");
        }
        if y % 10 == 0 {
            println!("row {} of {}", y, HEIGHT);
        }
        profile::end("ray casting row");
    }
    profile::end("ray casting");
    image::save_buffer(
        "image.png",
        &pixels.create_buffer(),
        WIDTH,
        HEIGHT,
        image::RGBA(8),
    ).unwrap();
    profile::end("main");
    profile::dump_flame();
}
