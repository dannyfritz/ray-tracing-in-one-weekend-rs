use nalgebra::normalize;
use ncollide3d::math::{Point, Vector};
use ncollide3d::query::Ray;
use std::f32;
use std::f32::consts::PI;
use utility::random::random_in_unit_disk;

pub struct Camera {
    origin: Vector<f32>,
    lower_left_corner: Vector<f32>,
    horizontal: Vector<f32>,
    vertical: Vector<f32>,
    lens_radius: f32,
    u: Vector<f32>,
    v: Vector<f32>,
    // w: Vector<f32>,
}

impl Camera {
    pub fn new(
        look_from: &Vector<f32>,
        look_at: &Vector<f32>,
        v_up: &Vector<f32>,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = *look_from;
        let w = normalize(&(look_from - look_at));
        let u = normalize(&(Vector::cross(v_up, &w)));
        let v = Vector::cross(&w, &u);
        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            // w,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray<f32> {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            Point::from_coordinates(self.origin + offset),
            self.lower_left_corner + u * self.horizontal + (1.0 - v) * self.vertical
                - self.origin
                - offset,
        )
    }
}
