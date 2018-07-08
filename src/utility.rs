#[allow(dead_code)]
pub mod random {
    use ncollide3d::math::Vector;
    use rand::distributions::StandardNormal;
    use rand::prelude::*;
    pub fn rand_gaussian() -> f32 {
        thread_rng().sample(StandardNormal) as f32
    }
    pub fn rand() -> f32 {
        thread_rng().gen()
    }
    pub fn random_in_unit_sphere() -> Vector<f32> {
        loop {
            let vec = 2.0 * Vector::new(rand(), rand(), rand()) - Vector::new(1.0, 1.0, 1.0);
            if vec.norm_squared() <= 1.0 {
                return vec;
            }
        }
    }
    pub fn random_in_unit_disk() -> Vector<f32> {
        loop {
            let p = 2.0 * Vector::<f32>::new(rand(), rand(), 0.0) - Vector::new(1.0, 1.0, 0.0);
            if p.norm_squared() <= 1.0 {
                return p;
            }
        }
    }
}
pub mod math {
    use nalgebra::normalize;
    use ncollide3d::math::Vector;
    pub fn reflect(vector: Vector<f32>, normal: Vector<f32>) -> Vector<f32> {
        vector - 2.0 * Vector::dot(&vector, &normal) * normal
    }
    pub fn refract(
        vector: Vector<f32>,
        normal: Vector<f32>,
        ni_over_nt: f32,
    ) -> Option<Vector<f32>> {
        let uv = normalize(&vector);
        let dt = uv.dot(&normal);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt())
        } else {
            None
        }
    }
}

pub mod profile {
    #[cfg(not(feature = "profile"))]
    pub fn start(_tag: &'static str) {}
    #[cfg(feature = "profile")]
    pub fn start(tag: &'static str) {
        flame::start(tag);
    }

    #[cfg(not(feature = "profile"))]
    pub fn end(_tag: &'static str) {}
    #[cfg(feature = "profile")]
    pub fn end(tag: &'static str) {
        flame::end(tag);
    }

    #[cfg(not(feature = "profile"))]
    pub fn dump_flame() {}
    #[cfg(feature = "profile")]
    pub fn dump_flame() {
        use std::fs::File;
        flame::dump_html(&mut File::create("profile.html").unwrap()).unwrap();
    }
}
