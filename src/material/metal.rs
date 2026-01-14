use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, util::rand::PCG32RNG,
    vec3::Vec3,
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut PCG32RNG) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector(rng));

        let scattered = Ray::new(rec.p, reflected);
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
