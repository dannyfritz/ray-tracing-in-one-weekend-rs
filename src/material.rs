use hitable::HitRecord;
use rand::{thread_rng, Rng};
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
        Lambertian { albedo }
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
        Metal { albedo, fuzz }
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Attenuation, Ray)> {
        let (outward_normal, ni_over_nt, cosine) =
            if Vec3::dot(&ray.direction(), &hit_record.normal) > 0.0 {
                (
                    -1.0 * hit_record.normal,
                    self.ref_idx,
                    self.ref_idx * Vec3::dot(&ray.direction(), &hit_record.normal)
                        / ray.direction().length(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.ref_idx,
                    -1.0 * Vec3::dot(&ray.direction(), &hit_record.normal)
                        / ray.direction().length(),
                )
            };
        let refracted_opt = Vec3::refract(&ray.direction(), &outward_normal, ni_over_nt);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflected = Vec3::reflect(&ray.direction(), &hit_record.normal);
        if refracted_opt.is_some()
            && thread_rng().gen_range(0.0, 1.0) > schlick(cosine, self.ref_idx)
        {
            Some((attenuation, Ray::new(hit_record.p, refracted_opt.unwrap())))
        } else {
            Some((attenuation, Ray::new(hit_record.p, reflected)))
        }
    }
}
