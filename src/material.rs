use hitable::HitRecord;
use ray::Ray;
use vec::Vec3;

pub type Attenuation = Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Attenuation, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Attenuation, Ray)> {
        let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
        Some((self.albedo, Ray::new(hit_record.p, target - hit_record.p)))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Attenuation, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&ray.direction()), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
