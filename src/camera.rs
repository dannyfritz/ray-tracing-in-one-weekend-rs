use ray::Ray;
use std::f32;
use std::f32::consts::PI;
use vec::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, v_up: &Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = *look_from;
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&(Vec3::cross(v_up, &w)));
        let v = Vec3::cross(&w, &u);
        println!("{:?}", 2.0 * half_height * v);
        Camera {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + (1.0 - v) * self.vertical - self.origin,
        )
    }
}
