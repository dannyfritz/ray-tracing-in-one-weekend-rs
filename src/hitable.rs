use material::Material;
use ray::Ray;
use std::rc::Rc;
use vec::Vec3;

#[cfg(feature = "profile")]
use flame;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct World {
    pub hitables: Vec<Box<dyn Hitable>>,
}
impl World {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("world hit");
        self.hitables
            .iter()
            .fold((t_max, None), |(closest, hit), hitable| {
                if let Some(rec) = hitable.hit(r, t_min, closest) {
                    (rec.t, Some(rec))
                } else {
                    (closest, hit)
                }
            })
            .1
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("sphere hit");
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
