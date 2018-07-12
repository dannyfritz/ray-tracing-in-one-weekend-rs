use camera::Camera;
use material::{Dialectric, Lambertian, Material, Metal};
use ncollide3d::bounding_volume::{aabb, HasBoundingVolume, AABB};
use ncollide3d::math::{Isometry, Vector};
use ncollide3d::partitioning::{BVTCostFn, BVT};
use ncollide3d::query::{Ray, RayCast, RayIntersection};
use ncollide3d::shape::Ball;
use utility::random::rand;
use {HEIGHT, WIDTH};

#[cfg(feature = "profile")]
use flame;

pub struct SceneObject {
    pub material: Box<dyn Material>,
    geometry: Box<dyn RayCast<f32>>,
    transform: Isometry<f32>,
}
impl SceneObject {
    pub fn new<M, G>(material: M, geometry: G, transform: Isometry<f32>) -> SceneObject
    where
        M: 'static + Material,
        G: 'static + RayCast<f32> + HasBoundingVolume<f32, AABB<f32>>,
    {
        SceneObject {
            material: Box::new(material),
            geometry: Box::new(geometry),
            transform,
        }
    }
    pub fn cast(&self, ray: &Ray<f32>) -> Option<RayIntersection<f32>> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("Scene Cast");
        self.geometry
            .toi_and_normal_with_ray(&self.transform, ray, false)
    }
}

pub struct ClosestRayTOICostFn<'a> {
    ray: &'a Ray<f32>,
}
impl<'a> ClosestRayTOICostFn<'a> {
    pub fn new(ray: &'a Ray<f32>) -> ClosestRayTOICostFn<'a> {
        ClosestRayTOICostFn { ray: ray }
    }
}
impl<'a> BVTCostFn<f32, SceneObject, AABB<f32>> for ClosestRayTOICostFn<'a> {
    type UserData = RayIntersection<f32>;
    fn compute_bv_cost(&mut self, bv: &AABB<f32>) -> Option<f32> {
        bv.toi_with_ray(&Isometry::identity(), self.ray, true)
    }
    fn compute_b_cost(&mut self, b: &SceneObject) -> Option<(f32, RayIntersection<f32>)> {
        b.cast(self.ray).map(|inter| (inter.toi, inter))
    }
}

fn create_bvt_tuple<M, G>(
    material: M,
    shape: G,
    transform: Isometry<f32>,
) -> (SceneObject, AABB<f32>)
where
    M: 'static + Material,
    G: 'static + RayCast<f32> + HasBoundingVolume<f32, AABB<f32>>,
{
    let aabb = aabb(&shape, &transform);
    (SceneObject::new(material, shape, transform), aabb)
}

#[allow(dead_code)]
pub fn structured_art_scene() -> (BVT<SceneObject, AABB<f32>>, Camera) {
    let mut hitables: Vec<(SceneObject, AABB<f32>)> = vec![];
    hitables.push(create_bvt_tuple(
        Lambertian::new(Vector::new(0.1, 0.5, 0.5)),
        Ball::new(1000.0f32),
        Isometry::new(Vector::new(0.0, -1000.0, 0.0), Vector::z()),
    ));
    hitables.push(create_bvt_tuple(
        Dialectric::new(1.5),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(-2.0, 1.0, 0.0), Vector::z()),
    ));
    hitables.push(create_bvt_tuple(
        Lambertian::new(Vector::new(1.0, 0.0, 0.0)),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(0.0, 1.0, 0.0), Vector::z()),
    ));
    hitables.push(create_bvt_tuple(
        Lambertian::new(Vector::new(0.0, 1.0, 0.0)),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(2.0, 1.0, 0.0), Vector::z()),
    ));
    let look_from = Vector::new(0.0, 1.0, 8.0);
    let look_at = Vector::new(0.0, 1.0, -1.0);
    let fov = 25.0;
    let distance_to_focus = 10000.0;
    let aperture = 0.0;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &Vector::new(0.0, 1.0, 0.0),
        fov,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        distance_to_focus,
    );
    (BVT::new_balanced(hitables), camera)
}

pub fn random_scene() -> (BVT<SceneObject, AABB<f32>>, Camera) {
    let mut hitables: Vec<(SceneObject, AABB<f32>)> = vec![];
    hitables.push(create_bvt_tuple(
        Lambertian::new(Vector::new(0.5, 0.5, 0.5)),
        Ball::new(1000.0f32),
        Isometry::new(Vector::new(0.0, -1000.0, 0.0), Vector::z()),
    ));
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = rand();
            let center = Vector::new(a as f32 + 0.9 + rand(), 0.2, b as f32 + 0.9 + rand());
            if (center - Vector::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    hitables.push(create_bvt_tuple(
                        Lambertian::new(Vector::new(
                            rand() * rand(),
                            rand() * rand(),
                            rand() * rand(),
                        )),
                        Ball::new(0.2f32),
                        Isometry::new(center, Vector::z()),
                    ));
                } else if choose_mat < 0.95 {
                    hitables.push(create_bvt_tuple(
                        Metal::new(
                            Vector::new(
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                            ),
                            0.5 + (1.0 + rand()),
                        ),
                        Ball::new(0.2f32),
                        Isometry::new(center, Vector::z()),
                    ));
                } else {
                    hitables.push(create_bvt_tuple(
                        Dialectric::new(1.5),
                        Ball::new(0.2f32),
                        Isometry::new(center, Vector::z()),
                    ));
                }
            }
        }
    }
    hitables.push(create_bvt_tuple(
        Dialectric::new(1.5),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(0.0, 1.0, 0.0), Vector::z()),
    ));
    hitables.push(create_bvt_tuple(
        Lambertian::new(Vector::new(0.4, 0.2, 0.1)),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(-4.0, 1.0, 0.0), Vector::z()),
    ));
    hitables.push(create_bvt_tuple(
        Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0),
        Ball::new(1.0f32),
        Isometry::new(Vector::new(4.0, 1.0, 0.0), Vector::z()),
    ));
    let look_from = Vector::new(10.0, 2.0, 3.0);
    let look_at = Vector::new(0.0, 0.0, -1.0);
    let fov = 25.0;
    let distance_to_focus = (look_from - look_at).norm();
    let aperture = 0.1;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &Vector::new(0.0, 1.0, 0.0),
        fov,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        distance_to_focus,
    );
    (BVT::new_balanced(hitables), camera)
}
