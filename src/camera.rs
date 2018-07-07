use rand::{thread_rng, Rng};
use ray::Ray;
use std::f32;
use std::f32::consts::PI;
use vec::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        v_up: &Vec3,
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
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&(Vec3::cross(v_up, &w)));
        let v = Vec3::cross(&w, &u);
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
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = rd.x() * self.u + rd.y() * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + (1.0 - v) * self.vertical
                - self.origin
                - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(
            thread_rng().gen_range(0.0, 1.0),
            thread_rng().gen_range(0.0, 1.0),
            0.0,
        ) - Vec3::new(1.0, 1.0, 0.0);
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}
