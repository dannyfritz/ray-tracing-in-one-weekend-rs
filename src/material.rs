use nalgebra::normalize;
use ncollide3d::bounding_volume::AABB;
use ncollide3d::math::Vector;
use ncollide3d::partitioning::BVT;
use ncollide3d::query::{Ray, RayIntersection};
use scene::{ClosestRayTOICostFn, SceneObject};
use utility::math::{reflect, refract};
use utility::random::{rand, random_in_unit_sphere};
use MAX_DEPTH;

#[cfg(feature = "profile")]
use flame;

pub type Attenuation = Vector<f32>;

fn no_color() -> Vector<f32> {
    Vector::new(0.0, 0.0, 0.0)
}

pub fn color(ray: &Ray<f32>, world: &BVT<SceneObject, AABB<f32>>, depth: u32) -> Vector<f32> {
    #[cfg(feature = "profile")]
    let _guard = flame::start_guard("color");
    if depth > MAX_DEPTH {
        return no_color();
    }
    let mut visitor = ClosestRayTOICostFn::new(ray);
    match world.best_first_search(&mut visitor) {
        Some((scene_object, ray_intersection)) => {
            match scene_object.material.scatter(&ray, ray_intersection) {
                Some((attenuation, scattered)) => attenuation.component_mul(&color(
                    &Ray::new(scattered.origin + scattered.dir * 0.001, scattered.dir),
                    world,
                    depth + 1,
                )),
                None => no_color(),
            }
        }
        None => {
            let unit_direction = normalize(&ray.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        intersection: RayIntersection<f32>,
    ) -> Option<(Attenuation, Ray<f32>)>;
}

pub struct Lambertian {
    albedo: Vector<f32>,
}
impl Lambertian {
    pub fn new(albedo: Vector<f32>) -> Lambertian {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        intersection: RayIntersection<f32>,
    ) -> Option<(Attenuation, Ray<f32>)> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("lambert scatter");
        let p = ray.origin + ray.dir * intersection.toi;
        let target = p.coords + intersection.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(p, target - p.coords)))
    }
}

pub struct Metal {
    albedo: Vector<f32>,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vector<f32>, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        intersection: RayIntersection<f32>,
    ) -> Option<(Attenuation, Ray<f32>)> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("metal scatter");
        let p = ray.origin + ray.dir * intersection.toi;
        let reflected = reflect(normalize(&ray.dir), intersection.normal);
        let scattered = Ray::new(p, reflected + self.fuzz * random_in_unit_sphere());
        if Vector::dot(&scattered.dir, &intersection.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dialectric {
    ref_idx: f32,
}
impl Dialectric {
    pub fn new(ref_idx: f32) -> Dialectric {
        Dialectric { ref_idx }
    }
}
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
impl Material for Dialectric {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        intersection: RayIntersection<f32>,
    ) -> Option<(Attenuation, Ray<f32>)> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("dialectric scatter");
        let p = ray.origin + ray.dir * intersection.toi;
        let (outward_normal, ni_over_nt, cosine) =
            if Vector::dot(&ray.dir, &intersection.normal) > 0.0 {
                (
                    -1.0 * intersection.normal,
                    self.ref_idx,
                    self.ref_idx * Vector::dot(&ray.dir, &intersection.normal) / ray.dir.norm(),
                )
            } else {
                (
                    intersection.normal,
                    1.0 / self.ref_idx,
                    -1.0 * Vector::dot(&ray.dir, &intersection.normal) / ray.dir.norm(),
                )
            };
        let refracted_opt = refract(ray.dir, outward_normal, ni_over_nt);
        let attenuation = Vector::new(1.0, 1.0, 1.0);
        let reflected = reflect(ray.dir, intersection.normal);
        if refracted_opt.is_some() && rand() > schlick(cosine, self.ref_idx) {
            Some((attenuation, Ray::new(p, refracted_opt.unwrap())))
        } else {
            Some((attenuation, Ray::new(p, reflected)))
        }
    }
}
