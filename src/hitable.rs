use ray::Ray;
use vec::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct World {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        self.hitables.iter().fold(None, |hit, hitable| {
            if let Some(rec) = hitable.hit(r, t_min, closest) {
                closest = rec.t;
                Some(rec)
            } else {
                hit
            }
        })
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, r: f32) -> Sphere {
        Sphere {
            center: center,
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                });
            }
        }
        return None;
    }
}
